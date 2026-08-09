use clap::{Parser, Subcommand};

use crate::dir_init::ensure_lpnl_directories;
use crate::lpnl_init::init_lpnl;
use crate::lpnl_remove::remove_lpnl;
use crate::stats::current_machine_stats;
use crate::error::{report_dir_init, report_init, report_remove};

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
    Stats,
    Init    { #[arg(long)] default: bool },
    Remove  { #[arg(long)] force: bool },
}

fn main() {
    match ensure_lpnl_directories() {
        Ok(_) => {},
        Err(e) => report_dir_init(e),
    }
    let args = Args::parse();
    match args.command {
        Commands::Stats => println!("{}", current_machine_stats()),
        Commands::Init { default } => {
            match init_lpnl(default) {
                Ok(_) => (),
                Err(e) => {report_init(e);}
            }
        },
        Commands::Remove { force } => {
            match remove_lpnl(force) {
                Ok(_) => (),
                Err(e) => {report_remove(e);}
            }
        },
        
    }
}