pub struct Solution {}

impl Solution {
    pub fn merge_characters(s: String, k: i32) -> String {
        let mut indices = [s.len(); 26];
        let mut result = String::new();
        let k = k as usize;

        for i in 0..s.len() {
            let c = (s.as_bytes()[i] - b'a') as usize;
            if indices[c] != s.len() && result.len() - indices[c] <= k {
                continue;
            }
            indices[c] = result.len();
            result.push((c as u8 + b'a') as char);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3853() {
        assert_eq!(
            Solution::merge_characters("abca".to_string(), 3),
            "abc".to_string()
        );
        assert_eq!(
            Solution::merge_characters("aabca".to_string(), 2),
            "abca".to_string()
        );
        assert_eq!(
            Solution::merge_characters("yybyzybz".to_string(), 2),
            "ybzybz".to_string()
        );
    }
}

fn main() {}
