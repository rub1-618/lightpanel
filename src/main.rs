use std::{path::PathBuf};
use clap::{Parser, Subcommand};

use crate::error::LpnlError;

mod lpnl_setup;
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
    proceed_with_check(dir_init::ensure_lpnl_directories());
    match args.command {
        Commands::Setup => {
            let setup_message = proceed_with_str(lpnl_setup::nginx_setup());
            println!("{}", setup_message);
        }

        Commands::SetBackup { domain } =>
            proceed_with_check(lpnl_backup::set_backup(domain)),
        Commands::GetBackup { domain } => 
            proceed_with_check(lpnl_backup::get_backup(domain)),

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

        Commands::Enable { domain } =>
            proceed_with_check(lpnl_state_ctrl::enable_domain(domain)),
        Commands::Disable { domain } =>
            proceed_with_check(lpnl_state_ctrl::disable_domain(domain)),
        Commands::EnableAll => 
            proceed_with_check(lpnl_state_ctrl::enable_all()),
        Commands::DisableAll =>
            proceed_with_check(lpnl_state_ctrl::disable_all()),
        Commands::Status { domain } => {
            let status = proceed_with_state(lpnl_state_ctrl::get_status(domain));
            println!("\nStatus: {status:?}")
        },

        Commands::List          => {
            let enabled = proceed_with_str(lpnl_list::list_enabled());
            let disabled = proceed_with_str(lpnl_list::list_disabled());
            let backups = proceed_with_str(lpnl_list::list_backups());
            println!("\n{}\n------------------------------------------\n\n{}\n------------------------------------------\n\n{}",
             enabled, disabled, backups)
        },
        Commands::ListEnabled   => {
            let enabled = proceed_with_str(lpnl_list::list_enabled());
            println!("\n{}", enabled)
        },
        Commands::ListDisabled   => {
            let disabled = proceed_with_str(lpnl_list::list_disabled());
            println!("\n{}", disabled)
        },
        Commands::ListBackups   => {
            let backups = proceed_with_str(lpnl_list::list_backups());
            println!("\n{}", backups)
        },

        Commands::AddLoc { domain, location, root, proxy } => 
            proceed_with_check(lpnl_add_loc::add_loc( domain, location, root, proxy)),

        Commands::RemoveLoc { domain, location } => 
            proceed_with_check(lpnl_remove_loc::remove_loc( domain, location)),

        Commands::Stats         => println!("{}------------------------------------------\n\n    {}", 
                                    lpnl_stats::short_stats(), lpnl_stats::disk_stats()),
        Commands::ShortStats    => println!("{}", lpnl_stats::short_stats()),
        Commands::DiskStats     => println!("\n{}", lpnl_stats::disk_stats()),
    }
}

fn proceed_with_check(func: Result<(), LpnlError>) {
    match func {
        Ok(_) => {},
        Err(e) => error::report_error(e),
    }
}

fn proceed_with_str(func: Result<String, LpnlError>) -> String {
    match func {
        Ok(s) => return s,
        Err(e) => {error::report_error(e); "Error emitted.".to_string()},
    }
}

use crate::lpnl_state_ctrl::State;
fn proceed_with_state(func: Result<State, LpnlError>) -> State {
    match func {
        Ok(s) => return s,
        Err(e) => {error::report_error(e); State::Unknown},
    }
}