pub mod packet;

use std::io::{prelude::*, ErrorKind};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
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
        listener.set_nonblocking(true).unwrap();
        let (tx, rx) = mpsc::channel::<String>();
        let mut clients = vec![];

        loop {
            if let Ok((mut stream, _addr)) = listener.accept() {
                let tx = tx.clone();
                clients.push(stream.try_clone().unwrap());

                thread::spawn(move || loop {
                    let mut buffer = vec![0; 256];

                    match stream.read(&mut buffer) {
                        Ok(amt) => {
                            let result = &buffer[..amt];

                            let data = str::from_utf8(&result).unwrap();
                            tx.send(data.to_string()).unwrap();
                        }
                        Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                        Err(_) => {
                            break;
                        }
                    }
                });
            }
            if let Ok(msg) = rx.try_recv() {
                clients = clients
                    .into_iter()
                    .filter_map(|mut client| {
                        let mut buff = msg.clone().into_bytes();
                        buff.resize(256, 0);

                        client.write_all(&buff).map(|_| client).ok()
                    })
                    .collect::<Vec<_>>();
            }
        }
    }

    pub fn start_command_server() {
        println!(
            "
Started the TCP Stream on 127.0.0.1:7978

This stream is used to send commands to the game
"
        );

        let listener = TcpListener::bind("127.0.0.1:7978").unwrap();
        listener.set_nonblocking(true).unwrap();
        let (tx, rx) = mpsc::channel::<String>();
        let mut clients = vec![];

        loop {
            if let Ok((mut stream, _addr)) = listener.accept() {
                let tx = tx.clone();
                clients.push(stream.try_clone().unwrap());

                thread::spawn(move || loop {
                    let mut buffer = vec![0; 4];

                    match stream.read(&mut buffer) {
                        Ok(amt) => {
                            let result = &buffer[..amt];

                            let data = str::from_utf8(&result).unwrap();
                            tx.send(data.to_string()).unwrap();
                        }
                        Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                        Err(_) => {
                            break;
                        }
                    }
                });
            }
            if let Ok(msg) = rx.try_recv() {
                clients = clients
                    .into_iter()
                    .filter_map(|mut client| {
                        let mut buff = msg.clone().into_bytes();
                        buff.resize(4, 0);

                        client.write_all(&buff).map(|_| client).ok()
                    })
                    .collect::<Vec<_>>();
            }
        }
    }

    fn handle_connection(mut stream: TcpStream, buffer_size: usize) {
        loop {
            let mut buffer = vec![0; buffer_size];

            let amt = stream.read(&mut buffer).unwrap();
            let result = &buffer[..amt];

            let data = str::from_utf8(&result).unwrap();

            stream.write_all(data.as_bytes()).unwrap();
        }
    }
}
