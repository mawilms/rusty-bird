use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn start_server() -> Self {
        println!(
            "
Started the TCP Stream on 127.0.0.1:7878
"
        );

        Self {
            listener: TcpListener::bind("127.0.0.1:7878").unwrap(),
        }
    }

    pub fn listen(&self) {
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();

            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];

        stream.read_exact(&mut buffer).unwrap();

        let response = "HTTP/1.1 200 OK\r\n\r\n";

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
