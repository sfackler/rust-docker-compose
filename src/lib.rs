//! A wrapper over Docker compositions.
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, Child};
use std::time::{Duration, Instant};
use std::thread;

use serde_types::Container;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod serde_types;

/// A running docker composition.
///
/// The composition will be shut down when this type falls out of scope.
pub struct DockerComposition {
    compose_file: PathBuf,
    log_child: Child,
    ports: HashMap<String, HashMap<u16, u16>>,
}

impl Drop for DockerComposition {
    fn drop(&mut self) {
        let _ = self.log_child.kill();
        let _ = self.log_child.wait();
        let _ = run(compose_command(&self.compose_file, &["down"]));
    }
}

impl DockerComposition {
    /// Creates a new `Builder` to configure a new composition.
    pub fn builder() -> Builder {
        Builder {
            checks: vec![],
            timeout: Duration::from_secs(60),
        }
    }

    /// Returns the external port mapping of the specified service's internal
    /// port, if present.
    pub fn port(&self, service: &str, port: u16) -> Option<u16> {
        self.ports.get(service).and_then(|m| m.get(&port)).cloned()
    }
}

/// A builder to configure `DockerComposition`s.
pub struct Builder {
    checks: Vec<Box<Fn(&DockerComposition) -> bool>>,
    timeout: Duration,
}

impl Builder {
    /// Adds a service check which will be run when the composition is started.
    ///
    /// `Builder::build` will not return until all checks return `true`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use docker_compose::DockerComposition;
    /// use std::net::TcpStream;
    ///
    /// // Check if the my_server service has bound its port.
    /// fn port_bound(c: &DockerComposition) -> bool {
    ///     let port = c.port("my_server", 8080).unwrap();
    ///     TcpStream::connect(("localhost", port)).is_ok()
    /// }
    ///
    /// let composition = DockerComposition::builder()
    ///                       .check(port_bound)
    ///                       .build("docker-compose.yml")
    ///                       .unwrap();
    /// // We know that my_server has fully booted at this point.
    /// ```
    pub fn check<F>(&mut self, f: F) -> &mut Builder
        where F: Fn(&DockerComposition) -> bool + 'static
    {
        self.checks.push(Box::new(f));
        self
    }

    /// Sets the timeout for service checks.
    ///
    /// If all service checks have not returned `true` after this much time
    /// has elapsed, `Builder::build` will return an error.
    pub fn timeout(&mut self, timeout: Duration) -> &mut Builder {
        self.timeout = timeout;
        self
    }

    /// Boots the composition.
    ///
    /// This method will not return until all service checks have returned
    /// `true`.
    pub fn build<P>(&self, compose_file: P) -> Result<DockerComposition, Box<Error>>
        where P: AsRef<Path>
    {
        let compose_file = compose_file.as_ref().to_owned();
        try!(run(compose_command(&compose_file, &["build"])));
        try!(run(compose_command(&compose_file, &["up", "-d"])));

        let log_child = try!(self.start_log_child(&compose_file));
        let ports = try!(self.get_ports(&compose_file));

        let composition = DockerComposition {
            compose_file: compose_file,
            log_child: log_child,
            ports: ports,
        };

        try!(self.run_checks(&composition));

        Ok(composition)
    }

    fn start_log_child(&self, compose_file: &Path) -> Result<Child, Box<Error>> {
        let mut log_child = try!(compose_command(&compose_file, &["logs", "-f"])
                                     .stdout(Stdio::piped())
                                     .spawn());
        let stdout = log_child.stdout.take().unwrap();

        thread::spawn(move || {
            let stdout = BufReader::new(stdout);
            for line in stdout.lines() {
                let line = match line {
                    Ok(line) => line,
                    Err(_) => return,
                };

                info!("{}", line);
            }
        });

        Ok(log_child)
    }

    fn get_ports(&self,
                 compose_file: &Path)
                 -> Result<HashMap<String, HashMap<u16, u16>>, Box<Error>> {
        let containers = try!(run(compose_command(&compose_file, &["ps", "-q"])));
        let mut command = Command::new("docker");
        command.arg("inspect").stdin(Stdio::null());
        for container in containers.lines() {
            command.arg(container.trim());
        }

        let inspect = try!(run(command));
        let containers: Vec<Container> = try!(serde_json::from_str(inspect.trim()));

        let mut map = HashMap::new();
        for container in containers {
            let service = match container.config.labels.get("com.docker.compose.service") {
                Some(service) => service,
                None => {
                    return Err(format!("container {} missing com.docker.compose.service label",
                                       container.id).into());
                }
            };

            for (private, hosts) in container.network_settings.ports {
                let host = match hosts.into_iter().next() {
                    Some(host) => host,
                    None => continue,
                };

                let private = try!(private.split("/").next().unwrap().parse());
                let public = try!(host.host_port.parse());

                map.entry(service.clone())
                   .or_insert_with(|| HashMap::new())
                   .insert(private, public);
            }
        }

        Ok(map)
    }

    fn run_checks(&self, composition: &DockerComposition) -> Result<(), Box<Error>> {
        let start = Instant::now();

        for check in &self.checks {
            while !check(composition) {
                let now = Instant::now();
                if now - start > self.timeout {
                    return Err("timed out waiting for service checks".into());
                }
                thread::sleep(Duration::from_millis(50));
            }
        }

        Ok(())
    }
}

fn compose_command(compose_file: &Path, args: &[&str]) -> Command {
    let mut command = Command::new("docker-compose");
    command.arg("-f")
           .arg(compose_file)
           .stdin(Stdio::null());
    for arg in args {
        command.arg(arg);
    }
    command
}

fn run(mut command: Command) -> Result<String, Box<Error>> {
    let output = try!(command.output());

    if !output.status.success() {
        return Err(format!("command returned {:?}\nstdout:\n{}\nstderr\n{}",
                           output.status.code(),
                           String::from_utf8_lossy(&output.stdout),
                           String::from_utf8_lossy(&output.stderr))
                       .into());
    }

    String::from_utf8(output.stdout).map_err(|e| e.into())
}
