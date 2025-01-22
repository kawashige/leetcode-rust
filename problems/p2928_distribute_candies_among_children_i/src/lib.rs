pub struct Solution {}

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        let mut ways = 0;
        for i in 0..=n.min(limit) {
            for j in 0..=n.min(limit) {
                if n >= i + j && limit >= n - i - j {
                    ways += 1;
                }
            }
        }
        ways
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2928() {
        assert_eq!(Solution::distribute_candies(5, 2), 3);
        assert_eq!(Solution::distribute_candies(3, 3), 10);
    }
}
