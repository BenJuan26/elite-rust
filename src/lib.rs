extern crate dirs;

use std::path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_sol() {
        assert_eq!(get_star_system().unwrap(), String::from("Sol"));
    }
}

pub fn get_star_system_from_path(path: &path::Path) -> Result<String, String> {
    Ok(String::from("Sol"))
}

pub fn get_star_system() -> Result<String, String> {
    let mut log_dir: path::PathBuf = dirs::home_dir().ok_or("Couldn't determine user's home directory")?;
    log_dir.push(r"Saved Games/Frontier Developments/Elite Dangerous");

    get_star_system_from_path(log_dir.as_path())
}
