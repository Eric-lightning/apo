#[macro_use]
extern crate clap; // args analyze library.
mod cli; // imp cli module.

fn main() {
    let matches = cli::build_cli().get_matches();
    println!("HELLO")

}
