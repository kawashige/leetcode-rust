pub struct Solution {}

impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let mut count = [0; 2];
        let mut result = 0;

        for i in 0..s.len() {
            match s.as_bytes()[i] {
                b'0' if count[1] == 0 => {
                    count[0] += 1;
                }
                b'0' => {
                    result = result.max(count[0].min(count[1]) * 2);
                    count = [1, 0];
                }
                b'1' => {
                    count[1] += 1;
                }
                _ => unreachable!(),
            }
        }

        result.max(count[0].min(count[1]) * 2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2609() {
        assert_eq!(
            Solution::find_the_longest_balanced_substring("01000111".to_string()),
            6
        );
        assert_eq!(
            Solution::find_the_longest_balanced_substring("00111".to_string()),
            4
        );
        assert_eq!(
            Solution::find_the_longest_balanced_substring("111".to_string()),
            0
        );
    }
}
