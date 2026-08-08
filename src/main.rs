use std::env;

use crate::stats::current_machine_stats;

mod codegen;
mod stats;

fn main() {
    // possible arguments
    let init_arg: String  = "init".to_string();
    let stats_arg: String = "stats".to_string();
    let usage_arg: String = "usage".to_string();
    let log_arg: String   = "log".to_string();

    // arguments matching
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Expected an argument.");
    }

    else if args.len() == 2 {
        match &args[1] {

            init_arg  => todo!(),
            
            stats_arg => {
                let stats_str = current_machine_stats();
                println!("{stats_str}")
            },

            usage_arg => todo!(),

            log_arg   => todo!(),

            _                  => panic!("Unknown argument.")
        
        }
    }
}
