pub struct Solution {}

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut max = 0;
        let mut depth = 0;

        for b in s.as_bytes() {
            if b == &b'(' {
                depth += 1;
                max = max.max(depth);
            } else if b == &b')' {
                depth -= 1;
            }
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1614() {
        assert_eq!(Solution::max_depth("(1+(2*3)+((8)/4))+1".to_string()), 3);
        assert_eq!(Solution::max_depth("(1)+((2))+(((3)))".to_string()), 3);
    }
}
