#![warn(clippy::all, clippy::pedantic)]
use clap::{load_yaml, App};

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
}
