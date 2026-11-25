pub fn help_fn() {
    println!("==============================================");
    println!("          BypassOS Package Manager");
    println!("==============================================");
    println!("Using:");
    println!("  bps install <name of package>    --> Install some package");
    println!("  bps update                       --> Update BypassOS");
    println!("  bps help                         --> Show this menu");
    println!("  ------------- In development -------------");
    println!("  bps remove <name of package>     --> Remove package");
}

pub fn error_output(error_text: &str) {
    println!("");
    println!("");
    println!("* * * * * * * * * * * * * * *");
    println!("Error: {error_text}");
    println!(" * * * * * * * * * * * * * * *");
    println!("");
    println!("");
}
