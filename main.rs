mod json_parser;

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
                    if a.len() > 2 {
                        a
                    } else {
                        error_output("The utility name cannot be shorter than 2 characters!");
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
                        if let Some(_json) = get_arr_of_vers(&text) {
                            
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
        _ => {}
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
