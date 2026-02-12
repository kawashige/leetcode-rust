pub struct Solution {}

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let mut result = 1;
        for i in 0..s.len() {
            let mut count = [0; 26];
            for j in (0..=i).rev() {
                count[(s.as_bytes()[j] - b'a') as usize] += 1;
                if (0..count.len())
                    .all(|k| count[k] == 0 || count[k] == count[(s.as_bytes()[i] - b'a') as usize])
                {
                    result = result.max(i - j + 1);
                }
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3713() {
        assert_eq!(Solution::longest_balanced("abbac".to_string()), 4);
        assert_eq!(Solution::longest_balanced("zzabccy".to_string()), 4);
        assert_eq!(Solution::longest_balanced("aba".to_string()), 2);
    }
}
