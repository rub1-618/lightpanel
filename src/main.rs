use clap::{Parser, Subcommand};

use crate::{error::report_error, lpnl_list::{list_backups, list_enabled}};

mod setup;
mod constants;
mod dir_init;
mod error;
mod lpnl_list;
mod lpnl_stats;
mod lpnl_init;
mod lpnl_remove;
// mod lpnl_add;

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
    Setup,
    Init    { #[arg(long)] default: bool },
    Remove  { #[arg(long)] force: bool },

    // AddLocation,

    List,
    ListEnabled,
    ListBackups,

    Stats,
    ShortStats,
    DiskStats,
}

fn main() {
    let args = Args::parse();

    match dir_init::ensure_lpnl_directories() {
        Ok(_) => {},
        Err(e) => report_error(e), 
    } // works because inside the func it process::exits with code (1)

    match args.command {
        Commands::Setup => {
            match setup::nginx_setup() {
                Ok(_) => (),
                Err(e) => report_error(e)
            }
        }
        Commands::Init { default } => {
            match lpnl_init::init_lpnl(default) {
                Ok(_) => (),
                Err(e) => report_error(e)
            }
        },
        Commands::Remove { force } => {
            match lpnl_remove::remove_lpnl(force) {
                Ok(_) => (),
                Err(e) => report_error(e)
            }
        },

        // Commands::AddLocation   => lpnl_add::add(),

        Commands::List          => {
            let enabled = match list_enabled() {
                Ok(e) => e,
                Err(e) => return report_error(e)
            };
            let backups = match list_backups() {
                Ok(b) => b,
                Err(e) => return report_error(e)
            };
            println!("\n{}\n------------------------------------------\n\n{}", enabled, backups)
        },
        Commands::ListEnabled   => {
            let enabled = match list_enabled() {
                Ok(e) => e,
                Err(e) => return report_error(e)
            };
            println!("\n{}", enabled)
        },
        Commands::ListBackups   => {
            let backups = match list_backups() {
                Ok(b) => b,
                Err(e) => return report_error(e)
            };
            println!("\n{}", backups)
        },

        Commands::Stats         => println!("{}------------------------------------------\n\n    {}", 
                                    lpnl_stats::short_stats(), lpnl_stats::disk_stats()),
        Commands::ShortStats    => println!("{}", lpnl_stats::short_stats()),
        Commands::DiskStats     => println!("\n{}", lpnl_stats::disk_stats()),
    }
}