use env_logger;

use docker_compose::DockerComposition;
use std::io::Read;
use std::net::TcpStream;

fn main() {
    env_logger::init();

    let docker = DockerComposition::builder()
        .check(check_port)
        .build(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/docker-compose.yml"
        ))
        .unwrap();
    let port = docker.port("test", 1234).unwrap();
    let mut stream = TcpStream::connect(("localhost", port)).unwrap();
    let mut out = String::new();
    stream.read_to_string(&mut out).unwrap();
    assert_eq!(out, "hello");
}

fn check_port(composition: &DockerComposition) -> bool {
    let port = composition.port("test", 1234).unwrap();
    match TcpStream::connect(("localhost", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}
