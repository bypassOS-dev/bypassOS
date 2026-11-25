use std::{io::{BufRead, BufReader}, path::Path};
use crate::checker::is_utility_exist;
use crate::usefull_fun::error_output::error_output;

pub fn check_database(path: &Path, utility: &String, package_data: &Vec<String>) -> (bool, i32){
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
            continue;
        };

        is_utility_download = true;

        let Some(num) = info.get(0) else {
            error_output("Database Error: missing package number in local database");
            std::process::exit(1);
        };

        let Ok(num) = num.trim().parse::<i32>() else {
            error_output("Database Error! Number of package inside local database was damaged and we couldn't parse number!");
            std::process::exit(1);
        };

        number = num;

        let Some(vers) = package_data.get(0) else {
            error_output("Package Error: missing version info inside server's database");
            std::process::exit(1);
        };

        let Some(vers_from_local) = info.get(2) else {
            error_output("Database Error: missing version info in local database");
            std::process::exit(1);
        };

        if vers == vers_from_local {
            println!("\n\nThis utility has already been downloaded!");
            std::process::exit(1);
        }
    }

    if number == 0 {
        number = max_num;
    }

    (is_utility_download, number)
}