pub mod packet;

use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::{str, thread};

pub struct Server {
    participants: Vec<SocketAddr>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            participants: Vec::new(),
        }
    }

    pub fn start_server(&mut self) {
        println!(
            "
Started the TCP Stream on 127.0.0.1:7878
"
        );

        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.participants.push(stream.peer_addr().unwrap());
                    if stream.peer_addr().unwrap() == self.participants[0] {
                        // Game Client
                        thread::spawn(|| {
                            Self::handle_connection(stream);
                        });
                    } else {
                        // External App Client
                        thread::spawn(|| {
                            Self::handle_connection(stream);
                        });
                    }
                }
                Err(_) => todo!(),
            }
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        loop {
            let mut buffer = vec![0; 2048];

            let amt = stream.read(&mut buffer).unwrap();
            let result = &buffer[..amt];

            let bla = str::from_utf8(&result).unwrap();
            //println!("{}", bla);

            let response = "HTTP/1.1 200 OK\r\n\r\n";

            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}
