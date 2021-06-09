#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
mod game;

use clap::{load_yaml, App};

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    game::Game::start().expect("Error while staring game");
}
