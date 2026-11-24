use std::fs;
use std::io;

pub fn repleace_str(path: &str, need_num: i32, need_str: &str) -> io::Result<()>{
    let content = fs::read_to_string(path)?;
    let mut replaced = false;

    let updated = content
        .lines()
        .map(|line| {
            if !replaced {
                let num_prefix: String = line
                    .chars()
                    .take_while(|c| c.is_ascii_digit())
                    .collect();

                if let Ok(num_from_file) = num_prefix.trim().parse::<i32>() {
                    if num_from_file == need_num {
                        replaced = true;
                        return need_str;
                    }
                }
            }
            line
        })
        .collect::<Vec<&str>>()
        .join("\n");
        
    let final_output = if content.ends_with("\n") {
        format!("{updated}\n")
    } else {
        updated
    };

    fs::write(path, final_output)
}