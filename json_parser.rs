fn _get_arr_of_vers(data: &str) -> Option<(Vec<String>, Vec<String>)>{
    let mut count = 0;
    let mut level = 0;
    let mut is_here_text = false;
    loop {
        if level == 0 && count > 1 {
            // return Some();
        }
        if let Some(n) = data.chars().nth(count) {
            count += 1;

            match n.to_string().as_str() {
                "{" => {
                    level += 1;
                },
                "}" => {
                    level += 1;
                },
                "\"" => {
                    is_here_text = !is_here_text;
                },
                _ => {}
            }
                
        } else {
            return None
        };
    }
}