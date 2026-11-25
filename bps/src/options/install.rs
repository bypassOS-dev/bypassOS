use crate::usefull_fun::{error_output::{error_output, help_fn}, help_functions::repleace_str};
use crate::json_parser::get_arr_of_vers;
use crate::options::useful_fn_4_options::check_db::check_database;

use std::io::Write;

pub fn install_package(args: Vec<String>) {
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
            let Ok(data_to_text) = response.into_string() else {
                error_output("Data from server is invalid and we couldn't parse it, sorry.");
                std::process::exit(1);
            };

            let Some(json) = get_arr_of_vers(&data_to_text) else {
                error_output("Data from server is invalid and we couldn't parse it to json, sorry.");
                std::process::exit(1);
            };

            let Some(data_about_util) = json.get(second_args) else {
                error_output("Data was damaget and we couldn't get information about package");
                std::process::exit(1);
            };

            let Some(version) = data_about_util.get(0) else {
                error_output("Data was damaget and we couldn't get information about package");
                std::process::exit(1);
            };
                                
            let path = std::path::Path::new("/var/lib/bps/installed_utility.txt");
            if !path.exists() {
                if let Some(parent) = path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }

                if let Err(_) = std::fs::File::create(path) {
                    error_output("We couldn't find and create file 'installed_utility.txt'!\nTry to create file yourself inside '/var/lib/bps/'");
                };
            }

            // Here we just check: Is utility inside user's database?
            // If we found utility then "is_utility_downloaded" == true and number == the line on which the utility is located
            // If utility don't downloaded then "is_utility_downloaded" == false and number == amont of ALL LINES!
            println!("[INFO] Checking local data base...");
            let (is_utility_downloaded, number) = check_database(path, second_args, data_about_util); 

            // Just Check for prety output
            if !is_utility_downloaded {
                println!("[INFO] Package not found. Downloading...");
            } else {
                println!("[INFO] Package found, but new version is available. So update...");
            }

            if let Some(target_url) = data_about_util.get(1) {
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
                                    let need_str = format!("{number}. {second_args} ---> {version}");
                                    if let Ok(_) = repleace_str(path, number, &need_str) {};
                                } else {
                                    let need_str = format!("{number}. {second_args} ---> {version}");
                                    if let Ok(mut file) = std::fs::File::options()
                                        .append(true)
                                        .create(true)
                                        .open(path) {
                                            let _ = writeln!(file, "{need_str}");
                                        }
                                }
                            }
                        }
                    },
                    Err(_) => {
                        error_output("We were unable to download file.\nMaybe you don't have internet?");
                    }
                }
            }
        },
        Err(_) => {}
    }
}