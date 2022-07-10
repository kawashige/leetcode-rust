pub struct Solution {}

impl Solution {
    pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let move_count =
            s.as_bytes()
                .iter()
                .zip(t.as_bytes().iter())
                .fold([0; 26], |mut moves, (b1, b2)| {
                    moves[((26 + (b2 - b'a') - (b1 - b'a')) % 26) as usize] += 1;
                    moves
                });

        (1..move_count.len())
            .all(|i| move_count[i] <= if i <= k as usize % 26 { 1 } else { 0 } + k as usize / 26)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1540() {
        assert!(!Solution::can_convert_string(
            "atmtxzjkz".to_string(),
            "tvbtjhvjd".to_string(),
            35
        ));
        assert!(Solution::can_convert_string(
            "input".to_string(),
            "ouput".to_string(),
            9
        ));
        assert!(!Solution::can_convert_string(
            "abc".to_string(),
            "bcd".to_string(),
            10
        ));
        assert!(Solution::can_convert_string(
            "aab".to_string(),
            "bbb".to_string(),
            27
        ));
    }
}
