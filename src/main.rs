use clap::{Parser, Subcommand};

use crate::stats::current_machine_stats;
use crate::commads::init_lpnl;
use crate::error::report_init;

mod error;
mod stats;
mod commads;

#[derive(Parser)]
#[command(version, name = "lpnl")]
struct Args {
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Stats,
    Init    { #[arg(long)] default: bool },
    Config  { },
}

fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Stats => println!("{}", current_machine_stats()),
        Commands::Init { default } => {
            match init_lpnl(default) {
                Ok(_) => (),
                Err(e) => {report_init(e);}
            }
        },
        Commands::Config {  } => todo!()
    }
}
