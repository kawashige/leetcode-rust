pub struct Solution {}

impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let mut prev_pos = 0;
        let mut time = word.len() as i32;

        for b in word.as_bytes() {
            let current_pos = (b - b'a') as i32;
            time += (prev_pos - current_pos)
                .abs()
                .min(26 - (prev_pos - current_pos).abs());
            prev_pos = current_pos;
        }

        time
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1974() {
        assert_eq!(Solution::min_time_to_type("abc".to_string()), 5);
        assert_eq!(Solution::min_time_to_type("bza".to_string()), 7);
        assert_eq!(Solution::min_time_to_type("zjpc".to_string()), 34);
    }
}
