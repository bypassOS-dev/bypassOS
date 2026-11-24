use crate::usefull_fun::error_output::error_output;
use std::process::Command;

pub fn update_package() {
    let url = "https://raw.githubusercontent.com/bypassOS-dev/storage_bypassOS/refs/heads/main/update.sh";
    println!("[INFO] Downloading update-file...");

    match ureq::get(url).call() {
        Ok(response) => {
            let text = response.into_string().expect("We can't read .sh file");

            let _ = Command::new("sh")
                .arg("-c")
                .arg(&text)
                .status()
                .expect("We couldn't run sh");
        },
        Err(_) => {
            error_output("We were unable to download file.\nMaybe you don't have internet?");
        }
    }
}