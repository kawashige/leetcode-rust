pub struct Solution {}

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }

        let mut open = Vec::new();
        let mut unlocked = Vec::new();

        for i in 0..s.len() {
            if locked.as_bytes()[i] == b'0' {
                unlocked.push(i);
            } else if s.as_bytes()[i] == b'(' {
                open.push(i);
            } else {
                if !open.is_empty() {
                    open.pop();
                } else if !unlocked.is_empty() {
                    unlocked.pop();
                } else {
                    return false;
                }
            }
        }

        let mut j = 0;
        for i in 0..open.len() {
            while j < unlocked.len() && unlocked[j] < open[i] {
                j += 1;
            }
            if unlocked.len() <= j {
                return false;
            }
            j += 1;
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2116() {
        assert!(!Solution::can_be_valid(
            "()()((()))((".to_string(),
            "111111101101".to_string()
        ));
        assert!(Solution::can_be_valid(
            "())()))()(()(((())(()()))))((((()())(())".to_string(),
            "1011101100010001001011000000110010100101".to_string(),
        ));
        assert!(Solution::can_be_valid(
            "))()))".to_string(),
            "010100".to_string()
        ));
        assert!(Solution::can_be_valid(
            "()()".to_string(),
            "0000".to_string()
        ));
        assert!(!Solution::can_be_valid(")".to_string(), "0".to_string()));
    }
}
