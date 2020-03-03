#[macro_use]
extern crate lazy_static;

pub mod star_system;
pub mod status;

use std::path;
use regex::Regex;

lazy_static! {
    static ref JOURNAL_FILE_REGEX: Regex = Regex::new(r"^Journal\.\d{12}\.\d{2}\.log$").unwrap();
    static ref HOME_DIR: path::PathBuf = dirs::home_dir().unwrap();
}
