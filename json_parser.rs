use std::collections::HashMap;

fn _get_arr_of_vers(data: &str) -> Option<(Vec<String>, Vec<String>)>{
    let mut count = 0;
    let mut level = 0;
    let mut is_here_text = false;


    let mut name: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
    let mut for_name = String::new();
    let mut version: HashMap<&str, &str> = HashMap::new();
    loop {
        if level == 0 && count > 1 {
            // return Some();
        }
        if let Some(n) = data.chars().nth(count) {
            count += 1;

            if n.is_whitespace() {
                continue;
            }
            match n.to_string().as_str() {
                ":" => {
                    continue;
               }
                "{" => {
                    level += 1;
                },
                "}" => {
                    level += 1;
                },
                "\"" => {
                    is_here_text = !is_here_text;
                },
                _ => {
                    if is_here_text == false{
                        continue;
                    }
                    let mut word= String::new();
                    let mut counter = count;
                    loop {
                        if let Some(char) = data.chars().nth(counter) {
                            if char.to_string() == "\"" {
                                count = counter - 1;
                                break;
                            }
                            word.push(char);
                            counter += 1;
                            continue;
                        }
                        
                    }

                    if level == 1 {
                        for_name.push_str(&word);
                    } else if word.as_str().starts_with('v') {

                    } else if word.as_str().starts_with('u') {

                    } else if word.as_str().starts_with('d') {

                    } else if word.as_str().starts_with('s') {

                    } else {
                        
                    }
                }
            }
                
        } else {
            return None
        };
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_work() {

    }
}
