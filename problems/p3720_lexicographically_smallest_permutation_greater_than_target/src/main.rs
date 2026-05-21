pub struct Solution {}

impl Solution {
    pub fn lex_greater_permutation(s: String, target: String) -> String {
        let mut count = [0; 26];
        for b in s.as_bytes() {
            count[(b - b'a') as usize] += 1;
        }

        let mut result = String::new();

        for i in 0..target.len() {
            let index = (target.as_bytes()[i] - b'a') as usize;
            if let Some(j) = (index + 1..26).find(|j| 0 < count[*j]) {
                let mut permutation_string = target[..i].to_string();
                permutation_string.push((j as u8 + b'a') as char);
                for k in 0..26 {
                    let c = count[k] - if k == j { 1 } else { 0 };
                    for _ in 0..c {
                        permutation_string.push((k as u8 + b'a') as char);
                    }
                }
                result = permutation_string;
            }
            if count[index] == 0 {
                break;
            }
            count[index] -= 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3720() {
        assert_eq!(
            Solution::lex_greater_permutation("abc".to_string(), "bba".to_string()),
            "bca".to_string()
        );
        assert_eq!(
            Solution::lex_greater_permutation("leet".to_string(), "code".to_string()),
            "eelt".to_string()
        );
        assert_eq!(
            Solution::lex_greater_permutation("baba".to_string(), "bbaa".to_string()),
            "".to_string()
        );
    }
}

fn main() {}
