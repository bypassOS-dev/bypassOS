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
        i += 2;

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

    #[test]
    fn multiple_packages() {
        let json = r#"{
          "pkg1": { "version": "1.0", "url": "http://a.com", "depends": "dep1", "sha256": "111" },
          "pkg2": { "version": "2.0", "url": "http://b.com", "depends": "", "sha256": "222" }
        }"#;

        let res = get_arr_of_vers(json).unwrap();
        assert_eq!(res.len(), 2);
        assert_eq!(res["pkg1"][0], "1.0");
        assert_eq!(res["pkg2"][1], "http://b.com");
        assert_eq!(res["pkg1"][2], "dep1");
        assert_eq!(res["pkg2"][3], "222");
    }


        #[test]
    fn test_four_short_packages() {
        let json = r#"{
  "pkg1": {
    "version": "1.0",
    "url": "http://a.org",
    "depends": "",
    "sha256": "111"
  },
  "pkg2": {
    "version": "2.1",
    "url": "http://b.org",
    "depends": "pkg1",
    "sha256": "222"
  },
  "pkg3": {
    "version": "0.5",
    "url": "http://c.org",
    "depends": "pkg1, pkg2",
    "sha256": "333"
  },
  "pkg4": {
    "version": "3.0",
    "url": "http://d.org",
    "depends": "",
    "sha256": "444"
  }
}"#;

        let res = get_arr_of_vers(json).unwrap();

        let mut expected = HashMap::new();
        expected.insert("pkg1".to_string(), vec!["1.0".to_string(), "http://a.org".to_string(), "".to_string(), "111".to_string()]);
        expected.insert("pkg2".to_string(), vec!["2.1".to_string(), "http://b.org".to_string(), "pkg1".to_string(), "222".to_string()]);
        expected.insert("pkg3".to_string(), vec!["0.5".to_string(), "http://c.org".to_string(), "pkg1, pkg2".to_string(), "333".to_string()]);
        expected.insert("pkg4".to_string(), vec!["3.0".to_string(), "http://d.org".to_string(), "".to_string(), "444".to_string()]);

        assert_eq!(res, expected);
    }

}