pub struct Solution {}

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let s = s.chars().rev().collect::<String>();
        let mut num = 0;
        let mut count = 0;
        for i in 0..s.len().min(30) {
            if s.as_bytes()[i] == b'1' {
                if (k as usize) < num + 2_usize.pow(i as u32) {
                    break;
                }
                count += 1;
                num += 2_usize.pow(i as u32);
            }
        }

        (s.as_bytes().iter().filter(|b| b == &&b'0').count() + count) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2311() {
        assert_eq!(Solution::longest_subsequence("1001010".to_string(), 5), 5);
        assert_eq!(Solution::longest_subsequence("00101001".to_string(), 1), 6);
    }
}
