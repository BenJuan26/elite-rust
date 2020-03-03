use std::path;
use std::fs;
use std::io;
use std::io::BufRead;
use std::error::Error;

use super::JOURNAL_FILE_REGEX;
use super::HOME_DIR;

pub fn get_from_path(path: &path::Path) -> Result<String, Box<dyn Error>> {
    let files = fs::read_dir(path)?;
    let mut found = false;
    let mut system = String::new();

    for entry in files {
        let entry = entry?;
        let file_name_osstr = entry.file_name();
        let file_name = file_name_osstr.to_str();
        let file_name = match file_name {
            Some(file_name) => file_name,
            None => continue,
        };

        if !JOURNAL_FILE_REGEX.is_match(file_name) {
            continue;
        }

        let file = fs::File::open(entry.path())?;
        let lines = io::BufReader::new(file).lines();

        for line in lines {
            if let Ok(line) = line {
                let event = json::parse(line.as_str())?;
                if event["event"] == "FSDJump" || event["event"] == "Location" {
                    if let Some(system_option) = event["StarSystem"].as_str() {
                        system = String::from(system_option);
                        found = true;
                    }
                }
            }
        }
    }

    if !found {
        return Err(String::from("No location found in all journal files").into());
    }

    Ok(String::from(system))
}

pub fn get() -> Result<String, Box<dyn Error>> {
    let mut log_dir: path::PathBuf = HOME_DIR.clone();
    log_dir.push(r"Saved Games/Frontier Developments/Elite Dangerous");

    get_from_path(log_dir.as_path())
}