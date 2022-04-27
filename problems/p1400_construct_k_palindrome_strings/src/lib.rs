pub struct Solution {}

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            return false;
        }

        let odd_count = s
            .as_bytes()
            .iter()
            .fold([0; 26], |mut count, b| {
                count[(b - b'a') as usize] += 1;
                count
            })
            .iter()
            .filter(|c| **c % 2 == 1)
            .count();

        odd_count <= k as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1400() {
        assert!(Solution::can_construct("annablelle".to_string(), 2));
        assert!(!Solution::can_construct("leetcode".to_string(), 3));
        assert!(Solution::can_construct("true".to_string(), 4));
    }
}
