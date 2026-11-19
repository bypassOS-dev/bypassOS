mod json_parser;
mod checker;

use std::io::{BufRead, BufReader};

use json_parser::get_arr_of_vers;
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

        },
        "install" => {
            let second = match args.get(2) {
                Some(a) => {
                    if a.len() >= 2 {
                        a
                    } else {
                        error_output("The utility name cannot be shorter than 1 characters!");
                        help_fn();
                        std::process::exit(1);
                    }
                },
                _ => {
                    error_output("Didn't you add what u wanna download?");
                    help_fn();
                    std::process::exit(1);
                }
            };

            let url = "https://raw.githubusercontent.com/bypassOS-dev/storage_bypassOS/refs/heads/main/storage.json";

            match ureq::get(url).call() {
                Ok(response) => {
                    if let Ok(text) = response.into_string() {
                        if let Some(json) = get_arr_of_vers(&text) {
                            if let Some(_package_data) = json.get(second) {
                                // We need create a file that will keep data about all utilites version. Well... Inside /var/lib... for example
                                // and if utility isn't exist just add it to file (with version). If exist just update  
                                let path = std::path::Path::new("/var/lib/bps/installed_utility.txt");
                                if !path.exists() {
                                    if let Some(parent) = path.parent() {
                                        let _ = std::fs::create_dir_all(parent);
                                    }

                                    if let Err(_) = std::fs::File::create(path) {
                                        error_output("We couldn't find and create file 'installed_utility.txt'!\nTry to create file yourself inside '/var/lib/bps/'");
                                    };
                                }

                                if let Ok(content) = std::fs::File::open(path) {
                                    let reader = BufReader::new(content);

                                    for line in reader.lines() {
                                        if let Ok(line) = line {

                                        } 
                                    }
                                }

                            } else {
                                error_output("Sorry we were unable find your utility inside our storage.\n May you check utility name or send utility name to us and we'll add your utility?");
                            }
                        } else {
                            error_output("The file downloaded from the server is incorrect! Please let us know!");
                        }
                    }else {
                        error_output("Can't download file!");
                    };
                    
                }
                Err(_) => {
                    error_output("We were unable to download file.\nMaybe you don't have internet?");
                }
            }
        }
        _ => {
            error_output("non-existent function");
            help_fn();
            std::process::exit(1);
        }
    }
}

fn help_fn() {
    println!("==============================================");
    println!("          BypassOS Package Manager");
    println!("==============================================");
    println!("Using:");
    println!("  bps install <name of package>    --> Install some package");
    println!("  bps remove <name of package>     --> Remove package");
    println!("  bps update                       --> Update BypassOS");
    println!("  bps check                        --> Check for updates");
    println!("  bps help                         --> Show this menu");
}

fn error_output(error_text: &str) {
    println!("");
    println!("");
    println!("* * * * * * * * * * * * * * *");
    println!("Error: {error_text}");
    println!(" * * * * * * * * * * * * * * *");
    println!("");
    println!("");
}