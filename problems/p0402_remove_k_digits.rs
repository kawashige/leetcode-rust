pub struct Solution {}

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        fn recurse(chars: &[char], k: usize) -> String {
            println!("chars: {:?}, k: {}", chars, k);
            if k == 0 {
                Default::default()
            } else if chars.len() == k {
                chars.iter().collect::<String>()
            } else {
                let pos = chars[0..(chars.len() - (k - 1))]
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, c)| *c)
                    .unwrap()
                    .0;
                format!("{}{}", chars[pos], recurse(&chars[(pos + 1)..], k - 1))
            }
        }

        if num.len() == k as usize {
            return "0".to_string();
        }

        let chars = num.chars().collect::<Vec<_>>();
        let result = recurse(&chars, chars.len() - k as usize)
            .trim_start_matches('0')
            .to_string();
        if result.is_empty() {
            "0".to_string()
        } else {
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0402() {
        assert_eq!(
            "1219".to_string(),
            Solution::remove_kdigits("1432219".to_string(), 3)
        );
        assert_eq!(
            "200".to_string(),
            Solution::remove_kdigits("10200".to_string(), 1)
        );
        assert_eq!(
            "0".to_string(),
            Solution::remove_kdigits("10".to_string(), 2)
        );
        assert_eq!(
            "0".to_string(),
            Solution::remove_kdigits("10".to_string(), 1)
        );
        assert_eq!(
            "0".to_string(),
            Solution::remove_kdigits("100".to_string(), 1)
        );
        assert_eq!(
            "11".to_string(),
            Solution::remove_kdigits("112".to_string(), 1)
        );
    }
}
