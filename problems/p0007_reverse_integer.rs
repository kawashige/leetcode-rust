pub struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let abs = x
            .abs()
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0);
        if x < 0 {
            abs * -1
        } else {
            abs
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_7() {
        let ret = Solution::reverse(123);
        assert_eq!(321, ret);
    }
}
