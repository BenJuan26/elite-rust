#[macro_use]
extern crate lazy_static;

use std::path;
use std::fs;
use std::io;
use std::io::BufRead;
use std::error::Error;
use regex::Regex;
use serde::{Serialize, Deserialize};

lazy_static! {
    static ref JOURNAL_FILE_REGEX: Regex = Regex::new(r"^Journal\.\d{12}\.\d{2}\.log$").unwrap();
    static ref HOME_DIR: path::PathBuf = dirs::home_dir().unwrap();
}

pub fn get_star_system_from_path(path: &path::Path) -> Result<String, Box<dyn Error>> {
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

pub fn get_star_system() -> Result<String, Box<dyn Error>> {
    let mut log_dir: path::PathBuf = HOME_DIR.clone();
    log_dir.push(r"Saved Games/Frontier Developments/Elite Dangerous");

    get_star_system_from_path(log_dir.as_path())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fuel {
    #[serde(rename = "FuelMain")]
    pub fuel_main: f64,
    #[serde(rename = "FuelReservoir")]
    pub fuel_reservoir: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub timestamp: String,
    pub event: String,
    #[serde(rename = "Flags")]
    pub raw_flags: u32,
    #[serde(rename = "Pips")]
    pub pips: [i32; 3],
    #[serde(rename = "FireGroup")]
    pub fire_group: i32,
    #[serde(rename = "GuiFocus")]
    pub gui_focus: i32,
    #[serde(rename = "Fuel")]
    pub fuel: Fuel,
    #[serde(rename = "Cargo")]
    pub cargo: f64,
    #[serde(rename = "Latitude")]
    pub latitude: Option<f64>,
    #[serde(rename = "Longitude")]
    pub longitude: Option<f64>,
    #[serde(rename = "Heading")]
    pub heading: Option<i32>,
    #[serde(rename = "Altitude")]
    pub altitude: Option<i32>,
}

pub fn get_status_from_path(path: &path::Path) -> Result<Status, Box<dyn Error>> {
    let mut path_buf = path.to_path_buf();
    path_buf.push("Status.json");

    let contents = fs::read_to_string(path_buf.to_str().ok_or("Couldn't convert PathBuf to str")?)?;
    let status: Status = serde_json::from_str(contents.as_str())?;

    Ok(status)
}

pub fn get_status() -> Result<Status, Box<dyn Error>> {
    let status = get_status_from_path(HOME_DIR.as_path())?;

    Ok(status)
}
