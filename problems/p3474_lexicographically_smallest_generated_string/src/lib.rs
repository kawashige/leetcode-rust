pub struct Solution {}

impl Solution {
    pub fn generate_string(str1: String, str2: String) -> String {
        let n = str1.len();
        let m = str2.len();
        let mut s = vec!['a'; n + m - 1];
        let mut fixed = vec![false; n + m - 1];

        for i in 0..n {
            if str1.as_bytes()[i] == b'T' {
                for j in 0..m {
                    let target = str2.as_bytes()[j] as char;
                    if fixed[i + j] && s[i + j] != target {
                        return Default::default();
                    }
                    fixed[i + j] = true;
                    s[i + j] = target;
                }
            }
        }

        for i in 0..n {
            if str1.as_bytes()[i] == b'F' {
                let mut not_equal = false;
                let mut righmost_unfixed = None;
                for j in (0..m).rev() {
                    let target = str2.as_bytes()[j] as char;
                    if target != s[i + j] {
                        not_equal = true;
                    }
                    if righmost_unfixed.is_none() && !fixed[i + j] {
                        righmost_unfixed = Some(i + j);
                    }
                }
                if !not_equal {
                    if let Some(k) = righmost_unfixed {
                        s[k] = 'b';
                    } else {
                        return Default::default();
                    }
                }
            }
        }

        s.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3474() {
        assert_eq!(
            Solution::generate_string("TFTF".to_string(), "ab".to_string()),
            "ababa".to_string()
        );
        assert_eq!(
            Solution::generate_string("TFTF".to_string(), "abc".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::generate_string("F".to_string(), "d".to_string()),
            "a".to_string()
        );
    }
}
