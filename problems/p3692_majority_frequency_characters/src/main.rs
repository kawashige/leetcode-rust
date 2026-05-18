pub struct Solution {}

impl Solution {
    pub fn majority_frequency_group(s: String) -> String {
        let mut freq = [0; 26];
        for b in s.as_bytes() {
            freq[(b - b'a') as usize] += 1;
        }
        let mut bucket = vec![vec![]; s.len() + 1];
        for i in 0..26 {
            if 0 < freq[i] {
                bucket[freq[i]].push((i as u8 + b'a') as char);
            }
        }
        let max_len = bucket.iter().map(|b| b.len()).max().unwrap();
        bucket
            .into_iter()
            .rev()
            .find(|b| b.len() == max_len)
            .unwrap()
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3692() {
        assert_eq!(
            Solution::majority_frequency_group("aaabbbccdddde".to_string()),
            "ab".to_string()
        );
        assert_eq!(
            Solution::majority_frequency_group("abcd".to_string()),
            "abcd".to_string()
        );
        assert_eq!(
            Solution::majority_frequency_group("pfpfgi".to_string()),
            "fp".to_string()
        );
    }
}

fn main() {}
