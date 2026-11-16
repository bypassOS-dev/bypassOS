fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args[1].as_str().len() < 3 {
        println!("");
        println!("");
        println!("* * * * * * * * * * * * * * *");
        println!("Error: non-existent function");
        println!(" * * * * * * * * * * * * * * *");
        println!("");
        println!("");

        help_fn();

        std::process::exit(1);
    }

    let command = args[1].as_str();

    match command {
        "qwe" => {},
        "qweqwe" => {},
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