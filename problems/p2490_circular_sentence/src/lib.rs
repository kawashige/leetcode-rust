pub struct Solution {}

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let words = sentence.split_ascii_whitespace().collect::<Vec<_>>();
        for i in 0..words.len() {
            if words[i].as_bytes()[words[i].len() - 1] != words[(i + 1) % words.len()].as_bytes()[0]
            {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2490() {
        assert!(Solution::is_circular_sentence(
            "leetcode exercises sound delightful".to_string()
        ));
        assert!(Solution::is_circular_sentence("eetcode".to_string()));
        assert!(!Solution::is_circular_sentence(
            "Leetcode is cool".to_string()
        ));
    }
}
