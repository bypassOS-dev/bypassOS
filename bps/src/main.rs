mod json_parser;
mod checker;
pub mod options;
pub mod usefull_fun;

use options::update::update_package;
use usefull_fun::error_output::{error_output, help_fn};

use crate::options::install::install_package;
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let command = match args.get(1) {
        Some(a) => {
            if a.len() < 3 {
                error_output("non-existent function");
                help_fn();
                std::process::exit(1);
            } else {
                a
            }
        },
        _ => {
            error_output("non-existent function");
            help_fn();
            std::process::exit(1);
        }
    };

    let command = command.as_str();

    match command {
        "update" => {
            update_package();
        },
        "install" => {
            install_package(args);
        },

        "help" => {
            help_fn();
        }
        _ => {
            error_output("non-existent function");
            help_fn();
            std::process::exit(1);
        }
    }
}