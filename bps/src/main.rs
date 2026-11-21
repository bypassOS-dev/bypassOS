mod json_parser;
mod checker;
mod help_functions;

use std::io::{BufRead, BufReader};
use help_functions::repleace_str;
use checker::is_utility_exist;
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
            let second_args = match args.get(2) {
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
            println!("[INFO] Downloading file with supported utilites...");
            match ureq::get(url).call() {
                Ok(response) => {
                    if let Ok(text) = response.into_string() {
                        if let Some(json) = get_arr_of_vers(&text) {
                            if let Some(package_data) = json.get(second_args) {
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
                                println!("[INFO] Checking local data base...");
                                if let Ok(content) = std::fs::File::open(path) {
                                    let reader = BufReader::new(content);
                                    
                                    let mut is_utility_downloaded: bool = false;

                                    let mut number: i32 = 0;
                                    let mut utility_name: String = String::new();
                                    let mut version: String = String::new();
                                    for line in reader.lines() {
                                        if let Ok(line) = line {
                                            if let Some(info) = is_utility_exist(line, second_args) {
                                                is_utility_downloaded = true;
                                                if let Some(name) = info.get(1) {
                                                    utility_name = name.clone();
                                                }
                                                if let Some(version1) = info.get(2) {
                                                    version = version1.clone();
                                                }
                                                if let Some(num) = info.get(0) {
                                                    if let Ok(num) = num.trim().parse() {
                                                        number = num;
                                                    }
                                                }
                                                if let Some(vers) = package_data.get(0) {
                                                    if let Some(vers_from_local) = info.get(2) {
                                                        if vers == vers_from_local {
                                                            println!("\n\nThis utility has already been downloaded!");
                                                            std::process::exit(1);
                                                        }
                                                    }
                                                    
                                                };
                                            }
                                        } 
                                    }

                                    if !is_utility_downloaded {
                                        println!("[INFO] Package not found. Downloading...");
                                    } else {
                                        println!("[INFO] Package found, but new version is available. So update...");
                                    }


                                    if let Some(target_url) = package_data.get(1) {
                                        let archive_path = format!("/tmp/{}", second_args);

                                        match ureq::get(target_url).call() {
                                            Ok(response) => {
                                                let mut file = std::fs::File::create(&archive_path).expect("Failed to create temp file");
                                                std::io::copy(&mut response.into_reader(), &mut file).expect("Failed to download");

                                                println!("[INFO] Extracting archive to system...");

                                                let status = std::process::Command::new("tar")
                                                    .args(["-xzf", &archive_path, "-C", "/"])
                                                    .status()
                                                    .expect("Extract Error");

                                                if status.success() {
                                                    println!("[OK] Package {} successfully installed!", second_args);
                                                    if let Some(path) = path.to_str() {
                                                        if is_utility_downloaded {
                                                            let need_str = format!("{number}. {utility_name} ---> {version}");
                                                            if let Ok(_) = repleace_str(path, number, &need_str) {};
                                                        } else {

                                                        }
                                                    }
                                                }
                                            },
                                            Err(_) => {
                                                error_output("We were unable to download file.\nMaybe you don't have internet?");
                                            }
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
