pub struct Solution {}

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        let mut ways = 0;

        for i in 0..=limit.min(n) {
            let r = n - i;
            if limit < r && r + 1 <= (r - limit) * 2 {
                continue;
            }
            ways += (r + 1 - if limit < r { (r - limit) * 2 } else { 0 }) as i64
        }

        ways
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2929() {
        assert_eq!(Solution::distribute_candies(5, 2), 3);
        assert_eq!(Solution::distribute_candies(3, 3), 10);
    }
}
