pub struct Solution {}

impl Solution {
    pub fn max_distinct(s: String) -> i32 {
        let mut count = 0;
        let mut used = [false; 26];

        for i in 0..s.len() {
            if !used[(s.as_bytes()[i] - b'a') as usize] {
                used[(s.as_bytes()[i] - b'a') as usize] = true;
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3760() {
        assert_eq!(Solution::max_distinct("abab".to_string()), 2);
        assert_eq!(Solution::max_distinct("abcd".to_string()), 4);
        assert_eq!(Solution::max_distinct("aaaa".to_string()), 1);
    }
}

fn main() {}
