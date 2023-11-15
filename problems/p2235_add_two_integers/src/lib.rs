pub struct Solution {}

impl Solution {
    pub fn sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2235() {
        assert_eq!(Solution::sum(12, 5), 17);
        assert_eq!(Solution::sum(-10, 4), -6);
    }
}
