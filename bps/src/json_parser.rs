use std::collections::HashMap;

pub fn get_arr_of_vers(data: &str) -> Option<HashMap<String, Vec<String>>>{
    //===================================================================
    let mut is_version_here = false;
    let mut is_url_here = false;
    let mut is_depends_here = false;
    let mut is_sha256_here = false;
    //==========================================
    let mut is_version_ready = false;
    let mut is_url_ready = false;
    let mut is_depends_ready = false;
    let mut is_sha256_ready = false;
    //=========================================
    let mut count = 0;
    let mut level = 0;  
    let mut is_here_text = false;
    let mut main: HashMap<String, Vec<String>> = HashMap::new();
    //=============================================================
    let mut for_name = String::new();
    let mut version = String::new();
    let mut url = String::new();
    let mut depends = String::new();
    let mut sha256 = String::new();
    //======================================================================
    loop {
        // 
        if is_depends_ready && is_sha256_ready && is_url_ready && is_version_ready {
            let mut vector: Vec<String> = Vec::new();
            vector.push(version.clone());
            vector.push(url.clone());
            vector.push(depends.clone());
            vector.push(sha256.clone());
            main.insert(for_name.clone(), vector.clone());

            is_version_ready = false;
            is_url_ready = false;
            is_depends_ready = false;
            is_sha256_ready = false;

        }

        // level == 0 mean what parser already found last "}"
        // && count > 1 mean what must hapens more than 1 operation
        if level == 0 && count > 1 {
            return Some(main.clone())
        }
        if let Some(n) = data.chars().nth(count) {
            count += 1;

            if n.is_whitespace() {
                continue;
            }
            match n.to_string().as_str() {
                // Skip - isn't need for us
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
                    continue;
                },
                "]" => {
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
                    word.push(n);
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
                    } else if level == 1 {
                        for_name.clear();
                        for_name.push_str(&word.clone());
                    } else if level == 2 {
                        match word.as_str() {
                            "version" => is_version_here = true,
                            "url" => is_url_here = true,
                            "depends" => is_depends_here = true,
                            "sha256" => is_sha256_here = true,
                            _ => {}
                        }
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
    
    let a = get_arr_of_vers(json).unwrap();
    let mut qwe = Vec::new();
    qwe.push("0.2.0".to_string());
    qwe.push("https://github.com/user-attachments/files/33284356/Archive.tar.gz".to_string());
    qwe.push("".to_string());
    qwe.push("1d37008822711cff8f156604217edf4b220d204691e79c4c23c308cb6dea6714".to_string());

    let mut qqq = HashMap::new();
    qqq.insert("blinddpi".to_string(), qwe);
    assert_eq!(
        a,
        qqq
    );
    }
}