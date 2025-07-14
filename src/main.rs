use clap::Parser;

mod cli;
mod core;
mod model;
mod storage;

fn main() {
    let parsed_args = cli::CliArgs::try_parse().unwrap_or_else(|e| e.exit());
}
