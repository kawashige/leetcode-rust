pub struct Solution {}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn is_palindrome(chars: &[char]) -> bool {
            let mut i = 0;
            let mut j = chars.len() - 1;
            while i < j {
                if chars[i] != chars[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            true
        }

        fn recurse(chars: &[char], head: Vec<String>, results: &mut Vec<Vec<String>>) {
            if chars.len() == 0 {
                results.push(head);
                return;
            }
            for i in 0..chars.len() {
                if i == 0 || is_palindrome(&chars[0..=i]) {
                    let mut new_head = head.clone();
                    new_head.push(chars[0..=i].iter().collect::<String>());
                    recurse(&chars[(i + 1)..], new_head, results);
                }
            }
        }

        let mut results = Vec::new();
        recurse(&s.chars().collect::<Vec<char>>(), Vec::new(), &mut results);
        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0131() {
        assert_eq!(
            vec![
                vec!["a".to_string(), "a".to_string(), "b".to_string()],
                vec!["aa".to_string(), "b".to_string()],
            ],
            Solution::partition("aab".to_string())
        );
        assert_eq!(
            vec![
                vec!["a".to_string(), "a".to_string(), "b".to_string()],
                vec!["aa".to_string(), "b".to_string()],
            ],
            Solution::partition("aabaa".to_string())
        );
    }
}
