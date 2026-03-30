pub struct Solution {}

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let mut start_chars = Vec::new();
        let mut target_chars = Vec::new();

        for i in 0..start.len() {
            match start.as_bytes()[i] {
                b'L' => start_chars.push((b'L', i)),
                b'R' => start_chars.push((b'R', i)),
                _ => {}
            }
            match target.as_bytes()[i] {
                b'L' => target_chars.push((b'L', i)),
                b'R' => target_chars.push((b'R', i)),
                _ => {}
            }
        }

        if start_chars.len() != target_chars.len() {
            return false;
        }

        (0..start_chars.len()).all(|i| {
            start_chars[i].0 == target_chars[i].0
                && if start_chars[i].0 == b'L' {
                    target_chars[i].1 <= start_chars[i].1
                } else {
                    start_chars[i].1 <= target_chars[i].1
                }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2337() {
        assert!(Solution::can_change(
            "_L__R__R_".to_string(),
            "L______RR".to_string()
        ));
        assert!(!Solution::can_change(
            "R_L_".to_string(),
            "__LR".to_string()
        ));
        assert!(!Solution::can_change("_R".to_string(), "R_".to_string()));
    }
}
