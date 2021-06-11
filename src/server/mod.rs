pub mod packet;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

pub struct Server;

impl Server {
    pub fn start_server() {
        println!(
            "
Started the TCP Stream on 127.0.0.1:7878
"
        );

        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            thread::spawn(|| {
                Self::handle_connection(stream.unwrap());
            });
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        loop {
            let mut buffer = vec![0; 2048];

            let amt = stream.read(&mut buffer).unwrap();
            let result = &buffer[..amt];

            let bla = str::from_utf8(&result).unwrap();
            println!("{}", bla);

            let response = "HTTP/1.1 200 OK\r\n\r\n";


            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}
