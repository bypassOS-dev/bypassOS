pub fn is_utility_exist(line: String, utility: String) -> Option<Vec<String>> {
    let line = line.trim();
    if line.starts_with('#') {
        return None;
    }

    if let Some((num, line)) = line.split_once(".") {
        if let Some((name, version)) = line.split_once(" ---> ") {
            if name == utility {
                let mut info = Vec::new();
                info.push(num.to_string());
                info.push(name.to_string());
                info.push(version.to_string());
                return Some(info);
            }
            return None;
        };
    };
    None
}