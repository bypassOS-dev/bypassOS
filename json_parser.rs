use std::collections::HashMap;

fn get_arr_of_vers(data: &str) -> Option<HashMap<String, Vec<String>>>{
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
    let mut main: HashMap<String, Vec<String>> = HashMap::new();


    let mut for_name = String::new();
    let mut version = String::new();
    let mut url = String::new();
    let mut depends = String::new();
    let mut sha256 = String::new();
    //======================================================================
    loop {
        if is_depends_ready && is_sha256_ready && is_url_ready && is_version_ready {
            let mut vector: Vec<String> = Vec::new();
            vector.push(version.clone());
            vector.push(url.clone());
            vector.push(depends.clone());
            vector.push(sha256.clone());
            main.insert(for_name.clone(), vector.clone());
        }
        println!(
            "name: '{}', v: {}, u: {}, d: {}, s: {}",
            for_name, is_version_ready, is_url_ready, is_depends_ready, is_sha256_ready
        );

        if level == 0 && count > 1 {
            return Some(main.clone())
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
                "[" => {
                    if is_depends_here {
                        is_depends_ready = true;
                        is_depends_here = false;
                    }
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
                                count = counter + 1;
                                is_here_text = false;
                                break;
                            }
                            word.push(char);
                            counter += 1;
                            continue;
                        } else {
                            return None;
                        }
                        
                    }

                    if is_version_here {
                        version.clear();
                        version.push_str(&word.clone());
                        is_version_ready = true;
                        is_version_here = false;
                    } else if is_url_here {
                        url.clear();
                        url.push_str(&word.clone());
                        is_url_ready = true;
                        is_url_here = false;
                    } else if is_depends_here {
                        depends.clear();
                        depends.push_str(&word.clone());
                        is_depends_ready = true;
                        is_depends_here = false;
                    }else if is_sha256_here {
                        sha256.clear();
                        sha256.push_str(&word.clone());
                        is_sha256_ready = true;
                        is_sha256_here = false;
                    }

                    if level == 1 {
                        word.clear();
                        for_name.push_str(&word.clone());
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
    let json = r#"{
  "blinddpi": {
    "version": "0.2.0",
    "url": "https://github.com/user-attachments/files/33284356/Archive.tar.gz",
    "depends": [],
    "sha256": "1d37008822711cff8f156604217edf4b220d204691e79c4c23c308cb6dea6714"
  }
}"#;

    if let Some(a) = get_arr_of_vers(&json) {
        println!("{:?}", a);
    };
    }
}
