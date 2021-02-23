pub struct Solution {}

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        fn recurse(bytes: &[u8], can_delete: bool) -> bool {
            let mut i = 0;
            let mut j = bytes.len() - 1;

            while i < j {
                if bytes[i] == bytes[j] {
                    i += 1;
                    j -= 1;
                    continue;
                }

                if can_delete {
                    return recurse(&bytes[i..j], false) || recurse(&bytes[(i + 1)..=j], false);
                } else {
                    return false;
                }
            }
            true
        }
        recurse(s.as_bytes(), true)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0680() {
        assert!(Solution::valid_palindrome("aguokepatgbnvfqmgmlcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupuculmgmqfvnbgtapekouga".to_string()));
        assert!(Solution::valid_palindrome("bddb".to_string()));
        assert!(Solution::valid_palindrome("aba".to_string()));
        assert!(!Solution::valid_palindrome("abccbadd".to_string()));
        assert!(Solution::valid_palindrome("abca".to_string()));
    }
}
