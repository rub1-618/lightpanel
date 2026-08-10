use clap::{Parser, Subcommand};

use crate::error::{report_error};

mod dir_init;
mod error;
mod stats;
mod lpnl_init;
mod lpnl_remove;

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
    Init    { #[arg(long)] default: bool },
    Remove  { #[arg(long)] force: bool },
    Stats,
}

fn main() {
    let args = Args::parse();

    match dir_init::ensure_lpnl_directories() {
        Ok(_) => {},
        Err(e) => report_error(e), 
    } // works because inside the func it process::exits with code (1)

    match args.command {
        Commands::Stats => println!("{}", stats::current_machine_stats()),
        Commands::Init { default } => {
            match lpnl_init::init_lpnl(default) {
                Ok(_) => (),
                Err(e) => {report_error(e);}
            }
        },
        Commands::Remove { force } => {
            match lpnl_remove::remove_lpnl(force) {
                Ok(_) => (),
                Err(e) => {report_error(e);}
            }
        }
    }
}