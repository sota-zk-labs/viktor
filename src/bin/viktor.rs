use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};

use std::env;
use viktor::config::{Config, Mode};
use viktor::task::run_task;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[clap(short, long)]
    version: bool,
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
enum Commands {
    Start {
        #[clap(long, default_value = "http://0.0.0.0:9944")]
        madara_url: Option<String>,
    },
    Deploy,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    if args.version {
        println!(env!("APP_VERSION"));
        return Ok(());
    }

    match args.command {
        None => {
            Cli::command().print_help()?;
            Ok(())
        }
        Some(command) => match command {
            Commands::Start { madara_url } => {
                let config = Config::new(madara_url, Mode::Run);

                println!("[🍬] Start running task!");
                run_task(config).await?;

                Ok(())
            }
            Commands::Deploy => {
                let config = Config::new(None, Mode::Deploy);

                println!("[🍬] Start running task!");
                run_task(config).await?;

                Ok(())
            }
        },
    }
}
