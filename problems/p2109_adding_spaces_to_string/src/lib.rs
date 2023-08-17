pub struct Solution {}

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut result = String::new();
        let mut j = 0;

        for i in 0..s.len() {
            if j < spaces.len() && spaces[j] as usize == i {
                result.push(' ');
                j += 1;
            }
            result.push(s.as_bytes()[i] as char);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2109() {
        assert_eq!(
            Solution::add_spaces("LeetcodeHelpsMeLearn".to_string(), vec![8, 13, 15]),
            "Leetcode Helps Me Learn".to_string()
        );
        assert_eq!(
            Solution::add_spaces("icodeinpython".to_string(), vec![1, 5, 7, 9]),
            "i code in py thon".to_string()
        );
        assert_eq!(
            Solution::add_spaces("spacing".to_string(), vec![0, 1, 2, 3, 4, 5, 6]),
            " s p a c i n g".to_string()
        );
    }
}
