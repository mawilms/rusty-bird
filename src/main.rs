#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
mod game;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            "
Rusty-bird by Marius Wilms <info@mariuswilms.dev>

Use one of these flags:

start -> Start the game"
        )
    } else if args[1].to_lowercase() == "start" {
        game::Game::start().expect("Error while staring game");
    }
}
