pub struct Solution {}

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut count = 0;
        for c in s.chars() {
            match c {
                '(' | '*' => count += 1,
                ')' => count -= 1,
                _ => unreachable!(),
            }
            if count < 0 {
                return false;
            }
        }
        count = 0;
        for c in s.chars().rev() {
            match c {
                ')' | '*' => count += 1,
                '(' => count -= 1,
                _ => unreachable!(),
            }
            if count < 0 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0678() {
        assert!(!Solution::check_valid_string("(((((*(()((((*((**(((()()*)()()()*((((**)())*)*)))))))(())(()))())((*()()(((()((()*(())*(()**)()(())".to_string()));
        assert!(Solution::check_valid_string("*".to_string()));
        assert!(Solution::check_valid_string("()".to_string()));
        assert!(Solution::check_valid_string("(*)".to_string()));
        assert!(Solution::check_valid_string("(*))".to_string()));
    }
}
