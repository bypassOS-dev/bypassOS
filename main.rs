mod json_parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args[1].as_str().len() < 3 {
        error_output("non-existent function");

        help_fn();

        std::process::exit(1);
    }

    let command = args[1].as_str();

    match command {
        "update" => {

        },
        "install" => {
            if args[2].as_str().len() <= 1 {
                error_output("programm name must contain more than 1 chair");

                help_fn();
            }

            let url = "https://raw.githubusercontent.com/bypassOS-dev/storage_bypassOS/refs/heads/main/storage.json";

            match ureq::get(url).call() {
                Ok(response) => {
                    if let Ok(text) = response.into_string() {
                        
                        println!("{text}");
                    }else {
                        error_output("Can't download file!");
                    };
                    
                }
                Err(_) => {

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