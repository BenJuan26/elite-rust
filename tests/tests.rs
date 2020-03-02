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
fn it_returns_sol() {
    let path = DATA_DIR.clone();
    assert_eq!(elite_rust::get_star_system_from_path(path.as_path()).unwrap(), String::from("Sol"));
}