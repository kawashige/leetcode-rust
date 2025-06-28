pub struct Solution {}

impl Solution {
    pub fn min_anagram_length(s: String) -> i32 {
        for i in 1..s.len() {
            if s.len() % i != 0 {
                continue;
            }
            let mut count = [0; 26];
            for j in 0..i {
                count[(s.as_bytes()[j] - b'a') as usize] += 1;
            }

            let mut found = true;
            for j in (i..s.len()).step_by(i) {
                let mut remains = count.clone();
                for k in 0..i {
                    remains[(s.as_bytes()[j + k] - b'a') as usize] -= 1;
                    if remains[(s.as_bytes()[j + k] - b'a') as usize] < 0 {
                        found = false;
                        break;
                    }
                }
                if !found {
                    break;
                }
            }
            if found {
                return i as i32;
            }
        }

        s.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3138() {
        assert_eq!(Solution::min_anagram_length("abba".to_string()), 2);
        assert_eq!(Solution::min_anagram_length("cdef".to_string()), 4);
        assert_eq!(
            Solution::min_anagram_length("abcbcacabbaccba".to_string()),
            3
        );
    }
}
