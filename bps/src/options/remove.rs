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
    let mut found = false;

    // We already did this check while download but better check again:
    let bad_paths = [
        "/", "/bin", "/boot", "/dev", "/etc", "/home", 
        "/lib", "/lib64", "/opt", "/proc", "/root", 
        "/sbin", "/sys", "/tmp", "/usr", "/var"
    ];

    for line in reader.lines() {
        let Ok(line) = line else {
            println!("[WARNING] We couldn't delete some file!");
            continue;
        };

        let line = line.trim();
        
        if found {
            if line.starts_with("----------") {
                break;
            }

            let clean_path = line.trim_end_matches('/');
            if clean_path.is_empty() || !clean_path.starts_with('/') || bad_paths.contains(&clean_path) || clean_path.contains("..") {
                println!("[WARNING] We found the bad path inside /var/lib/bps/name_utility_for_remove and we assume that YOU edited this file. DON'T DO THAT NEVER! \nIf you don't edited the file that send feedback to us pls <3");
                continue;
            }

            if line.ends_with("/") {
                let _ = std::fs::remove_dir(line);
            } else {
                let _ = std::fs::remove_file(line);
            }
        }
        if line == target_header {
            found = true;
        }
    }
}