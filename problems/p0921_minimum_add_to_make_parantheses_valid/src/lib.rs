pub struct Solution {}

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        s.chars()
            .fold(vec![0, 0], |count, c| {
                if c == '(' {
                    vec![count[0] + 1, count[1]]
                } else if count[0] == 0 {
                    vec![count[0], count[1] + 1]
                } else {
                    vec![count[0] - 1, count[1]]
                }
            })
            .into_iter()
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0921() {
        assert_eq!(Solution::min_add_to_make_valid("())".to_string()), 1);
        assert_eq!(Solution::min_add_to_make_valid("(((".to_string()), 3);
        assert_eq!(Solution::min_add_to_make_valid("()".to_string()), 0);
        assert_eq!(Solution::min_add_to_make_valid("()))((".to_string()), 4);
    }
}
