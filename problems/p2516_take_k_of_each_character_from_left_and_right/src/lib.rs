pub struct Solution {}

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        if k == 0 {
            return 0;
        }
        if s.len() < 3 * k as usize {
            return -1;
        }

        let mut r = s.len() - 1;

        let mut count = vec![0; 3];
        count[(s.as_bytes()[r] - b'a') as usize] += 1;

        while 1 <= r && count.iter().any(|c| c < &k) {
            r -= 1;
            count[(s.as_bytes()[r] - b'a') as usize] += 1;
        }

        if count.iter().any(|c| c < &k) {
            return -1;
        }

        let mut minutes = s.len() - r;

        for l in 0..s.len() {
            count[(s.as_bytes()[l] - b'a') as usize] += 1;
            while r < s.len() && k < count[(s.as_bytes()[r] - b'a') as usize] {
                count[(s.as_bytes()[r] - b'a') as usize] -= 1;
                r += 1;
            }
            minutes = minutes.min(l + 1 + s.len() - r);
        }

        minutes as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2516() {
        assert_eq!(Solution::take_characters("bcca".to_string(), 1), 3);
        assert_eq!(Solution::take_characters("abc".to_string(), 1), 3);
        assert_eq!(Solution::take_characters("aabaaaacaabc".to_string(), 2), 8);
        assert_eq!(Solution::take_characters("a".to_string(), 1), -1);
        assert_eq!(Solution::take_characters("a".to_string(), 0), 0);
    }
}
