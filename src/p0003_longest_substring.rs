pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();
        let mut result = 0;
        for i in 0..chars.len() {
            let mut j = i + 1;
            while j < chars.len() {
                if chars[i..j].contains(&chars[j]) {
                    break;
                }
                j += 1;
            }
            if result < j - i {
                result = j - i;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::length_of_longest_substring("".to_string()));
        assert_eq!(1, Solution::length_of_longest_substring(" ".to_string()));
        assert_eq!(
            1,
            Solution::length_of_longest_substring("bbbbb".to_string())
        );
        assert_eq!(
            3,
            Solution::length_of_longest_substring("abcabcbb".to_string())
        );
    }
}
