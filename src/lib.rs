#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_sol() {
        assert_eq!(get_star_system(), String::from("Sol"));
    }
}

pub fn get_star_system() -> String {
    String::from("Sol")
}
