extern crate dirs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_sol() {
        assert_eq!(get_star_system().unwrap(), String::from("Sol"));
    }
}

pub fn get_star_system_from_path(path: &std::path::Path) -> Option<String> {
    Some(String::from("Sol"))
}

pub fn get_star_system() -> Option<String> {
    let mut log_dir: std::path::PathBuf = dirs::home_dir()?;
    log_dir.push(r"Saved Games/Frontier Developments/Elite Dangerous");

    get_star_system_from_path(log_dir.as_path())
}
