use std::io::BufRead;

use crate::usefull_fun::error_output::error_output;

pub fn remove(args: Vec<String>) {
    let Some(second_args) = args.get(2) else {
        error_output("You don't enter utility name!");
    };

    if second_args.len() < 2 {
        error_output("The utility name cannot be shorter than 2 characters!");
    }

    let Ok(file) = std::fs::File::open("/var/lib/bps/name_utility_for_remove") else {
        error_output("We couldn't open file '/var/lib/bps/name_utility_for_remove'!")
    };
    let reader = std::io::BufReader::new(file);

    let target_header = format!("----------{}----------", second_args);
    let mut found = false;
    let mut was_found = false;
    let mut file = Vec::new(); 

    // We already did this check while download but better check again:
    let bad_paths = [
        "/", "/bin", "/boot", "/dev", "/etc", "/home", 
        "/lib", "/lib64", "/opt", "/proc", "/root", 
        "/sbin", "/sys", "/tmp", "/usr", "/var",
        "/usr/bin", "/usr/lib", "/usr/local", "/usr/share", "/var/lib"
    ];

    println!("[INFO] Checking database...");
    for line in reader.lines() {
        let Ok(line) = line else {
            println!("[WARNING] We couldn't delete some file!");
            continue;
        };

        let line_string = line.trim();

        if line_string == target_header {
            found = true;
            was_found = true;
            continue;
        }

        if found {
            if line_string.starts_with("----------") {
                found = false;
                file.push(line);
                continue;

            }

            if line_string.is_empty() {
                continue;
            }

            let clean_path = line_string.trim_end_matches('/');
            if !clean_path.starts_with('/') || bad_paths.contains(&clean_path) || clean_path.contains("..") {
                println!("[WARNING] We found the bad path inside /var/lib/bps/name_utility_for_remove and we assume that YOU edited this file. DON'T DO THAT NEVER! \nIf you don't edited the file that send feedback to us pls <3");
                continue;
            }

            if line_string.ends_with("/") {
                let _ = std::fs::remove_dir_all(line_string);
            } else {
                let _ = std::fs::remove_file(line_string);
            }
        } else {
            file.push(line);
        }   
    }

    if !was_found {
        error_output("You entered a utility name that isn't even installed on your computer!");
    }
    println!("[INFO] Rewriting database...");
    let _ = std::fs::write("/var/lib/bps/name_utility_for_remove", file.join("\n"));

    println!("[OK] Package {} removed!", second_args);
}