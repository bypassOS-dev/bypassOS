use std::collections::HashMap;

pub fn get_arr_of_vers(data: &str) -> Option<HashMap<String, Vec<String>>>{
    let mut main: HashMap<String, Vec<String>> = HashMap::new();
    let parts: Vec<&str> = data.split('"').collect();

    let mut i = 1;

    while i < parts.len() {
        let name = parts[i].to_string();
        i += 4;

        let version = parts[i].to_string();
        i += 4;

        let url = parts[i].to_string();
        i += 4;

        let depends = parts[i].to_string();
        i += 4;

        let sha256 = parts[i].to_string();
        i += 4;

        main.insert(name, vec![version, url, depends, sha256]);
    }

    Some(main)
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
    "depends": "",
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