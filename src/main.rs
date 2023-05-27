#![allow(dead_code)]

use clap::{Args, Parser, Subcommand};
mod commands;

#[derive(Parser)]
#[clap(about, author, version)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Args)]
struct NewArgs {
    problem: Option<u8>,
}

#[derive(Args)]
struct RunArgs {
    problem: Option<u8>,

    #[arg(short, long, default_value_t = false)]
    benchmark: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Handles the initialisation of a new Project Euler Problem
    New(NewArgs),

    /// Runs the solution to a problem
    Run(RunArgs),
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let value = Value::parse();

    match value.command {
        Commands::New(NewArgs { problem }) => commands::new::execute(problem),
        Commands::Run(RunArgs { problem, benchmark }) => commands::run::execute(problem, benchmark),
    }
}
