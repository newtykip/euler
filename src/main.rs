#![allow(dead_code)]

mod commands;

use std::panic;

use clap::{Args, Parser, Subcommand};
use euler::{Result, SILENT_PANIC};

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
    #[clap(alias = "n")]
    New(NewArgs),

    /// Runs the solution to a problem
    #[clap(alias = "r")]
    Run(RunArgs),
}

#[tokio::main]
pub async fn main() -> Result<()> {
    panic::set_hook(Box::new(move |panic_info| {
        if unsafe { SILENT_PANIC } {
            std::process::exit(0);
        } else {
            println!("{}", panic_info.to_string());
        }
    }));

    let value = Value::parse();

    match value.command {
        Commands::New(NewArgs { problem }) => commands::new::execute(problem).await,
        Commands::Run(RunArgs { problem, benchmark }) => {
            commands::run::execute(problem, benchmark).await
        }
    }
}
