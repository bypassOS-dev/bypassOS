use crate::usefull_fun::{error_output::error_output, help_functions::repleace_str};
use crate::json_parser::get_arr_of_vers;
use crate::options::useful_fn_4_options::{check_db::check_database, check_remove_db::check_removing_db};

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
 
pub fn install_package(args: Vec<String>) {
    let Some(second_args) = args.get(2) else {
        error_output("You don't enter utility name!");
    };

    if second_args.len() < 2 {
        error_output("The utility name cannot be shorter than 1 characters!");
    }

    let url = "https://raw.githubusercontent.com/bypassOS-dev/storage_bypassOS/refs/heads/main/storage.json";
    println!("[INFO] Downloading file with supported utilites...");
    match ureq::get(url).call() {
        Ok(response) => {
            let Ok(data_to_text) = response.into_string() else {
                error_output("Data from server is invalid and we couldn't parse it, sorry.");
            };

            let Some(json) = get_arr_of_vers(&data_to_text) else {
                error_output("Data from server is invalid and we couldn't parse it to json, sorry.");
            };

            let Some(data_about_util) = json.get(second_args) else {
                error_output("Data was damaget and we couldn't get information about package");
            };

            let Some(version) = data_about_util.get(0) else {
                error_output("Data was damaget and we couldn't get information about package");
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
            println!("[INFO] Checking local database...");
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

                        println!("[INFO] check database for removing package...");
                        let del_path = "/var/lib/bps/name_utility_for_remove";
                        check_removing_db(del_path);

                        println!("[INFO] Extracting archive to system...");
                        let status = std::process::Command::new("tar")
                            .args(["-xzf", &archive_path, "-C", "/"])
                            .status()
                            .expect("Extract Error");

                        if status.success() {
                            println!("[INFO] Update file '/var/lib/bps/name_utility_for_remove'...");
                            let finish_writing = format!("\n----------{}----------", second_args);

                            let bad_paths = [
                                "/", "/bin", "/boot", "/dev", "/etc", "/home", 
                                "/lib", "/lib64", "/opt", "/proc", "/root", 
                                "/sbin", "/sys", "/tmp", "/usr", "/var"
                            ];

                            if let Ok(file) = File::open("/for_remove.txt")  {
                                let reader = BufReader::new(file);

                                let mut out_file = std::fs::OpenOptions::new()
                                    .create(true)
                                    .append(true)
                                    .open("/var/lib/bps/name_utility_for_remove")
                                    .unwrap();

                                writeln!(out_file, "{finish_writing}").unwrap();

                                for line in reader.lines() {
                                    let  Ok(line) = line else {
                                        continue;
                                    };
                                    
                                    let clean_lines = line.trim().trim_end_matches("/");

                                    if clean_lines.is_empty() || !clean_lines.starts_with("/") {
                                        continue;
                                    }

                                    if bad_paths.contains(&clean_lines) {
                                        println!("[WARNING] We found path that may destroed ur sistem but we save your pc!");
                                        continue;
                                    }

                                    if clean_lines.contains("..") {
                                        println!("[WARNING] We found path that may destroed ur sistem but we save your pc!");
                                        continue;
                                    }

                                    writeln!(out_file, "{clean_lines}").unwrap();
                                }

                                let _ = std::fs::remove_file("/for_remove.txt");

                            } else {
                                println!("[WARNING] We couldn't find the file responsible for deletion utilits file. Utility will work but we'll can't romove it.")
                            };

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