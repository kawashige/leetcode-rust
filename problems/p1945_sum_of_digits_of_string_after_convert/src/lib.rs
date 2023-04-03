pub struct Solution {}

impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut s = s
            .as_bytes()
            .iter()
            .map(|b| ((b - b'a') as i32 + 1).to_string())
            .collect::<String>();
        for _ in 0..k {
            s = s
                .as_bytes()
                .iter()
                .map(|b| (b - b'0') as i32)
                .sum::<i32>()
                .to_string();
        }

        s.parse::<i32>().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1945() {
        assert_eq!(Solution::get_lucky("iiii".to_string(), 1), 36);
        assert_eq!(Solution::get_lucky("leetcode".to_string(), 2), 6);
        assert_eq!(Solution::get_lucky("zbax".to_string(), 2), 8);
    }
}
