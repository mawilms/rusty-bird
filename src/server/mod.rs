pub mod packet;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

pub struct Server;

impl Server {
    pub fn start_state_server() {
        println!(
            "
Started the TCP Stream on 127.0.0.1:7878

You'll receive the state updates from this stream
"
        );

        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            thread::spawn(|| {
                Self::handle_connection(stream.unwrap());
            });
        }
    }

    pub fn start_command_server() {
        println!(
            "
Started the TCP Stream on 127.0.0.1:7878

This stream is used to send commands to the game
"
        );

        let listener = TcpListener::bind("127.0.0.1:7978").unwrap();

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

            let data = str::from_utf8(&result).unwrap();

            stream.write_all(data.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
