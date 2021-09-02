pub struct Solution {}

impl Solution {
    pub fn compress(s: String) -> Vec<(char, usize)> {
        s.chars().fold(Vec::new(), |mut v, c| {
            if !v.is_empty() && v.last().unwrap().0 == c {
                v.last_mut().unwrap().1 += 1;
            } else {
                v.push((c, 1));
            }
            v
        })
    }

    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let c_name = Self::compress(name);
        let c_typed = Self::compress(typed);

        c_name.len() == c_typed.len()
            && c_name
                .into_iter()
                .zip(c_typed.into_iter())
                .all(|(c, t)| c.0 == t.0 && c.1 <= t.1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0925() {
        assert!(Solution::is_long_pressed_name(
            "alex".to_string(),
            "aaleex".to_string()
        ));
        assert!(!Solution::is_long_pressed_name(
            "saeed".to_string(),
            "ssaaedd".to_string()
        ));
        assert!(Solution::is_long_pressed_name(
            "leelee".to_string(),
            "lleeelee".to_string()
        ));
        assert!(Solution::is_long_pressed_name(
            "laiden".to_string(),
            "laiden".to_string()
        ));
    }
}
