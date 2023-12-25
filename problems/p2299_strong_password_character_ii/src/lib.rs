use std::thread::panicking;

pub struct Solution {}

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        let special_charcters = "!@#$%^&*()-+".chars().collect::<Vec<_>>();
        8 <= password.len()
            && password.chars().any(|c| c.is_ascii_lowercase())
            && password.chars().any(|c| c.is_ascii_uppercase())
            && password.chars().any(|c| c.is_ascii_digit())
            && password.contains(&*special_charcters)
            && (1..password.len()).all(|i| password.as_bytes()[i - 1] != password.as_bytes()[i])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2299() {
        assert!(Solution::strong_password_checker_ii(
            "IloveLe3tcode!".to_string()
        ));
        assert!(!Solution::strong_password_checker_ii(
            "Me+You--IsMyDream".to_string()
        ));
        assert!(!Solution::strong_password_checker_ii("1aB!".to_string()));
    }
}
