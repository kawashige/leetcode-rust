pub struct Solution {}

impl Solution {
    pub fn residue_prefixes(s: String) -> i32 {
        let mut count = [0; 26];
        let mut distinct_chars = 0;
        let mut result = 0;

        for i in 0..s.len() {
            let c = (s.as_bytes()[i] - b'a') as usize;
            count[c] += 1;
            if count[c] == 1 {
                distinct_chars += 1;
            }
            if distinct_chars == (i + 1) % 3 {
                result += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3803() {
        assert_eq!(Solution::residue_prefixes("abc".to_string()), 2);
        assert_eq!(Solution::residue_prefixes("dd".to_string()), 1);
        assert_eq!(Solution::residue_prefixes("bob".to_string()), 2);
    }
}

fn main() {}
