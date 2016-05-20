extern crate env_logger;
extern crate docker_compose;

use docker_compose::DockerComposition;
use std::net::TcpStream;

fn main() {
    env_logger::init().unwrap();

    let _docker = DockerComposition::builder()
                      .check(check_port)
                      .build("tests/docker-compose.yml")
                      .unwrap();
}

fn check_port(composition: &DockerComposition) -> bool {
    let port = composition.port("test", 1234).unwrap();
    match TcpStream::connect(("localhost", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}
