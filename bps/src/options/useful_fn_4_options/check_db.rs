use std::{io::{BufRead, BufReader}, path::Path};
use crate::checker::is_utility_exist;
use crate::usefull_fun::error_output::error_output;

pub fn check_database(path: &Path, utility: &String ) {
    let Ok(content) = std::fs::File::open(path) else {
        error_output("We couldn't open a file with installed utility. Try run with root privileges, pls");
        std::process::exit(1);
    };

    let reader = BufReader::new(content);

    let mut is_utility_download: bool = false;

    let mut number: i32 = 0;
    let mut max_num= 1;

    for line in reader.lines() {
        max_num += 1;

        let Ok(line) = line else {
            error_output("We couldn't read file. Try run with root privileges, pls");
            std::process::exit(1);
        };

        let Some(info) = is_utility_exist(line, utility) else {
            error_output("Unknown error. Try run with root privileges, pls - that should help!");
            std::process::exit(1);
        };

        is_utility_download = true;

        let Some(num) = info.get(0) else {
            error_output("Unknown error");
            std::process::exit(1);
        };
    }
    // if let Ok(content) = std::fs::File::open(path) {
    //     let reader = BufReader::new(content);
                                    
    //     let mut is_utility_downloaded: bool = false;

    //     let mut number: i32 = 0;
    //     let mut max_num= 0;

    //     for line in reader.lines() {
    //         max_num += 1;
    //         if let Ok(line) = line {
    //             if let Some(info) = is_utility_exist(line, second_args) {
    //                 is_utility_downloaded = true;
    //                 if let Some(num) = info.get(0) {
    //                     if let Ok(num) = num.trim().parse() {
    //                         number = num;
    //                     }
    //                 }
    //                 if let Some(vers) = package_data.get(0) {
    //                     if let Some(vers_from_local) = info.get(2) {
    //                         if vers == vers_from_local {
    //                             println!("\n\nThis utility has already been downloaded!");
    //                             std::process::exit(1);
    //                         }
    //                     }
                                                
    //                 };
    //             }
    //         } 
    //     }
    // } else {

    // }
}