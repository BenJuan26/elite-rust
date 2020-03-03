#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;

use elite_rust;

lazy_static! {
    static ref DATA_DIR: PathBuf = get_data_dir();
}

fn get_data_dir() -> PathBuf {
    let mut path = PathBuf::from(file!());
    path.pop();
    path.push("data");

    path
}

#[test]
fn it_returns_star_system() {
    let path = DATA_DIR.clone();
    assert_eq!(elite_rust::get_star_system_from_path(path.as_path()).unwrap(), String::from("Sol"));
}

#[test]
fn it_returns_status() {
    let status = elite_rust::get_status_from_path(DATA_DIR.as_path()).unwrap();
    assert_eq!(status.raw_flags, 18939917);
    assert_eq!(status.pips[1], 8);
    assert_eq!(status.timestamp, String::from("2020-03-03T05:03:21Z"));
}
