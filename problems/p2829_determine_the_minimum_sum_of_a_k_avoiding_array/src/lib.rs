pub struct Solution {}

impl Solution {
    pub fn minimum_sum(n: i32, k: i32) -> i32 {
        (1..=k / 2).take(n.min(k / 2) as usize).sum::<i32>()
            + (k..).take((n - n.min(k / 2)) as usize).sum::<i32>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2829() {
        assert_eq!(Solution::minimum_sum(5, 4), 18);
        assert_eq!(Solution::minimum_sum(2, 6), 3);
    }
}
