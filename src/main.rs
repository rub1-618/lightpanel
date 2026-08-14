use std::{path::PathBuf};
use clap::{Parser, Subcommand};

mod setup;
mod constants;
mod validation;
mod commands;
mod dir_init;
mod error;
mod lpnl_list;
mod lpnl_stats;
mod lpnl_init;
mod lpnl_remove;
mod lpnl_state_ctrl;
mod lpnl_add_loc;
mod lpnl_remove_loc;
mod lpnl_backup;

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

    SetBackup   { domain: Option<String> },
    GetBackup   { domain: Option<String> },

    Init        { 
        domain: Option<String>, 
        #[arg(long)]
        root: Option<PathBuf>,
        #[arg(long)]
        port: Option<u16>,
        #[arg(long)] default: bool 
    },
    Remove      {
        domain: Option<String>,
        #[arg(long)] force: bool 
    },

    Enable      { domain: Option<String> },
    Disable     { domain: Option<String> },
    EnableAll,
    DisableAll,
    Status      { domain: Option<String> },

    List,
    ListEnabled,
    ListDisabled,
    ListBackups,
 
    AddLoc  {
        domain: Option<String>,
        #[arg(long)]
        location: Option<String>,
        #[arg(long)]
        root: Option<PathBuf>,
        #[arg(long)]
        proxy: Option<String>,
    },

    RemoveLoc  {
        domain: Option<String>,
        #[arg(long)]
        location: Option<String>,
    },

    Stats,
    ShortStats,
    DiskStats,
}

fn main() {
    let args = Args::parse();

    match dir_init::ensure_lpnl_directories() {
        Ok(_) => {},
        Err(e) => error::report_error(e), 
    } // works because inside the func it process::exits with code (1)

    match args.command {
        Commands::Setup => {
            match setup::nginx_setup() {
                Ok(_) => (),
                Err(e) =>error:: report_error(e)
            }
        }

        Commands::SetBackup { domain } => {
            match lpnl_backup::set_backup(domain) {
                Ok(_) => (),
                Err(e) =>error:: report_error(e)
            }
        },
        Commands::GetBackup { domain } => {
            match lpnl_backup::get_backup(domain) {
                Ok(_) => (),
                Err(e) =>error:: report_error(e)
            }
        },

        Commands::Init { domain, root, port, default } => {
            match lpnl_init::init_lpnl(domain, root, port, default) {
                Ok(_) => (),
                Err(e) => error::report_error(e)
            }
        },
        Commands::Remove { domain, force } => {
            match lpnl_remove::remove_lpnl(domain, force) {
                Ok(_) => (),
                Err(e) => error::report_error(e)
            }
        },

        Commands::Enable { domain } => {
            match lpnl_state_ctrl::enable_domain(domain) {
                Ok(_) => (),
                Err(e) => error::report_error(e),
            }
        },
        Commands::Disable { domain } => {
            match lpnl_state_ctrl::disable_domain(domain) {
                Ok(_) => (),
                Err(e) => error::report_error(e),
            }
        },
        Commands::EnableAll => {
            match lpnl_state_ctrl::enable_all() {
                Ok(_) => (),
                Err(e) => error::report_error(e),
            }
        },
        Commands::DisableAll => {
            match lpnl_state_ctrl::disable_all() {
                Ok(_) => (),
                Err(e) => error::report_error(e),
            }
        },
        Commands::Status { domain } => {
            match lpnl_state_ctrl::get_status(domain) {
                Ok(s) => println!("\nStatus: {:?}", s),
                Err(e) => error::report_error(e),
            }
        },

        Commands::List          => {
            let enabled = match lpnl_list::list_enabled() {
                Ok(e) => e,
                Err(e) => return error::report_error(e)
            };
            let disabled = match lpnl_list::list_disabled() {
                Ok(b) => b,
                Err(e) => return error::report_error(e)
            };
            let backups = match lpnl_list::list_backups() {
                Ok(b) => b,
                Err(e) => return error::report_error(e)
            };
            println!("\n{}\n------------------------------------------\n\n{}\n------------------------------------------\n\n{}",
             enabled, disabled, backups)
        },
        Commands::ListEnabled   => {
            let enabled = match lpnl_list::list_enabled() {
                Ok(e) => e,
                Err(e) => return error::report_error(e)
            };
            println!("\n{}", enabled)
        },
        Commands::ListDisabled   => {
            let enabled = match lpnl_list::list_disabled() {
                Ok(e) => e,
                Err(e) => return error::report_error(e)
            };
            println!("\n{}", enabled)
        },
        Commands::ListBackups   => {
            let backups = match lpnl_list::list_backups() {
                Ok(b) => b,
                Err(e) => return error::report_error(e)
            };
            println!("\n{}", backups)
        },

        Commands::AddLoc { domain, location, root, proxy } => {
            match lpnl_add_loc::add_loc( domain, location, root, proxy) {
                Ok(a) => a,
                Err(e) => return error::report_error(e),
            }
        },

        Commands::RemoveLoc { domain, location } => {
            match lpnl_remove_loc::remove_loc( domain, location) {
                Ok(a) => a,
                Err(e) => return error::report_error(e),
            }
        },

        Commands::Stats         => println!("{}------------------------------------------\n\n    {}", 
                                    lpnl_stats::short_stats(), lpnl_stats::disk_stats()),
        Commands::ShortStats    => println!("{}", lpnl_stats::short_stats()),
        Commands::DiskStats     => println!("\n{}", lpnl_stats::disk_stats()),
    }
}