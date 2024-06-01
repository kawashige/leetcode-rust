pub struct Solution {}

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        s.as_bytes()
            .windows(2)
            .fold(0, |acc, b| acc + (b[0] as i32 - b[1] as i32).abs())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3110() {
        assert_eq!(Solution::score_of_string("hello".to_string()), 13);
        assert_eq!(Solution::score_of_string("zaz".to_string()), 50);
    }
}
