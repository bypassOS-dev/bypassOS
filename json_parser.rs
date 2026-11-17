use std::collections::HashMap;

fn _get_arr_of_vers(data: &str) -> Option<(Vec<String>, Vec<String>)>{
    //===================================================================
    let mut is_version_here = false;
    let mut is_url_here = false;
    let mut is_depends_here = false;
    let mut is_sha256_here = false;

    let mut is_version_ready = false;
    let mut is_url_ready = false;
    let mut is_depends_ready = false;
    let mut is_sha256_ready = false;

    let mut count = 0;
    let mut level = 0;
    let mut is_here_text = false;
    let mut main: HashMap<&str, Vec<&str>> = HashMap::new();


    let mut for_name = String::new();
    let mut version = String::new();
    let mut url = String::new();
    let mut depends = String::new();
    let mut sha256 = String::new();
    //======================================================================
    loop {
        if is_depends_ready && is_sha256_ready && is_url_ready && is_version_ready {

        }
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
                    level -= 1;
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

                    if is_version_here {
                        version.clear();
                        version.push_str(&word.clone());
                        is_version_ready = true;
                    } else if is_url_here {
                        url.clear();
                        url.push_str(&word.clone());
                        is_url_ready = true;
                    } else if is_depends_here {
                        depends.clear();
                        depends.push_str(&word.clone());
                        is_depends_ready = true;
                    }else if is_sha256_here {
                        sha256.clear();
                        sha256.push_str(&word.clone());
                        is_sha256_ready = true;
                    }

                    if level == 1 {
                        for_name.push_str(&word);
                    } else if word.as_str().starts_with('v') && level == 2 {
                        is_version_here = true;
                    } else if word.as_str().starts_with('u') && level == 2 {
                        is_url_here = true;
                    } else if word.as_str().starts_with('d') && level == 2 {
                        is_depends_here = true;
                    } else if word.as_str().starts_with('s') && level == 2 {
                        is_sha256_here = true;
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
