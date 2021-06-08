use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub struct Server {}

impl Server {
    pub fn start_server() {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            Self::handle_connection(stream)
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];

        stream.read_exact(&mut buffer).unwrap();

        let response = "HTTP/1.1 200 OK\r\n\r\n";

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
