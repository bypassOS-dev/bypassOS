use std::io::BufRead;

use crate::usefull_fun::error_output::error_output;

pub fn remove(args: Vec<String>) {
    let Some(second_args) = args.get(2) else {
        error_output("You don't enter utility name!");
    };

    if second_args.len() < 2 {
        error_output("The utility name cannot be shorter than 1 characters!");
    }

    let Ok(file) = std::fs::File::open("/var/lib/bps/name_utility_for_remove") else {
        error_output("We couldn't open file '/var/lib/bps/name_utility_for_remove'!")
    };
    let reader = std::io::BufReader::new(file);

    let target_header = format!("----------{}----------", second_args);
    let mut is_our_package = false;
    let mut found = false;


    for line in reader.lines() {
        let Ok(line) = line else {
            println!("[WARNING] We couldn't delete some file!");
            continue;
        };

        let line = line.trim();
        

    }
}