pub struct Solution {}

impl Solution {
    pub fn process_str(s: String, k: i64) -> char {
        let mut str_len = vec![0; s.len()];
        let mut len = 0;
        for i in 0..s.len() {
            match s.as_bytes()[i] {
                b'*' => len = (len - 1).max(0),
                b'#' => len *= 2,
                b'%' => {}
                _ => len += 1,
            }
            str_len[i] = len;
        }

        if len < k + 1 {
            return '.';
        }

        let mut k = k;
        for i in (0..s.len()).rev() {
            match s.as_bytes()[i] {
                b'*' => len += 1,
                b'#' => {
                    if (len + 1) / 2 < k + 1 {
                        k -= len / 2;
                    }
                    len = (len + 1) / 2;
                }
                b'%' => k = len - k - 1,
                _ if k + 1 == len => return s.as_bytes()[i] as char,
                _ => len -= 1,
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3614() {
        assert_eq!(Solution::process_str("a#b%*".to_string(), 1), 'a');
        assert_eq!(Solution::process_str("cd%#*#".to_string(), 3), 'd');
        assert_eq!(Solution::process_str("z*#".to_string(), 0), '.');
    }
}

fn main() {}
