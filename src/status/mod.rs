use std::path;
use std::fs;
use std::error::Error;
use serde::{Serialize, Deserialize};

use super::HOME_DIR;

pub mod flags;

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

    #[serde(skip)]
    pub flags: flags::Flags,

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

impl Status {
    fn expand_flags(&mut self) {
        self.flags = flags::Flags::from(self.raw_flags);
    }
}

pub fn get_from_path(path: &path::Path) -> Result<Status, Box<dyn Error>> {
    let mut path_buf = path.to_path_buf();
    path_buf.push("Status.json");

    let contents = fs::read_to_string(path_buf.to_str().ok_or("Couldn't convert PathBuf to str")?)?;
    let mut status: Status = serde_json::from_str(contents.as_str())?;
    status.expand_flags();

    Ok(status)
}

pub fn get() -> Result<Status, Box<dyn Error>> {
    let status = get_from_path(HOME_DIR.as_path())?;

    Ok(status)
}
