#[macro_use]
extern crate clap;

use clap::{Shell};

include!("src/apo/cli.rs");

fn main() {
    let mut apo_app = build_cli();
    apo_app.gen_completions("apo", Shell::Bash,       "./compression/");
    apo_app.gen_completions("apo", Shell::Zsh,        "./compression/");
    apo_app.gen_completions("apo", Shell::Fish,       "./compression/");
    apo_app.gen_completions("apo", Shell::PowerShell, "./compression/");
}
