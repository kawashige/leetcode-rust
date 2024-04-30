pub struct Solution {}

impl Solution {
    pub fn closet_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let mut l = start_index as usize;
        let mut r = start_index as usize;

        for i in 0..words.len() {
            if words[l] == target || words[r] == target {
                return i as i32;
            }
            l = (words.len() + l - 1) % words.len();
            r = (r + 1) % words.len();
            if l == start_index as usize || r == start_index as usize {
                break;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2515() {
        assert_eq!(
            Solution::closet_target(
                vec![
                    "hello".to_string(),
                    "i".to_string(),
                    "am".to_string(),
                    "leetcode".to_string(),
                    "hello".to_string()
                ],
                "hello".to_string(),
                1
            ),
            1
        );
        assert_eq!(
            Solution::closet_target(
                vec!["a".to_string(), "b".to_string(), "leetcode".to_string()],
                "leetcode".to_string(),
                0
            ),
            1
        );
        assert_eq!(
            Solution::closet_target(
                vec!["i".to_string(), "eat".to_string(), "leetcode".to_string()],
                "ate".to_string(),
                0
            ),
            -1
        );
    }
}
