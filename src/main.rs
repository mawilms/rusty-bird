#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
mod game;
mod server;

use std::{env, thread};

use server::Server;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            "
Rusty-bird by Marius Wilms <info@mariuswilms.dev>

Use one of these flags:

start -> Start the game"
        );
    } else if args[1].to_lowercase() == "start" {
        let state_thread = thread::spawn(|| {
            Server::start_state_server();
        });
        let command_thread = thread::spawn(|| {
            Server::start_command_server();
        });

        game::Game::start().expect("Error while staring game");

        state_thread.join().unwrap();
        command_thread.join().unwrap();
    }
}
