pub struct Solution {}

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        (1i64..=(n as i64))
            .take_while(|i| i * (i + 1) / 2 <= n as i64)
            .last()
            .unwrap_or(0) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0441() {
        assert_eq!(65535, Solution::arrange_coins(2147483647));
        assert_eq!(0, Solution::arrange_coins(0));
        assert_eq!(1, Solution::arrange_coins(1));
        assert_eq!(2, Solution::arrange_coins(5));
        assert_eq!(3, Solution::arrange_coins(8));
        assert_eq!(3, Solution::arrange_coins(6));
    }
}
