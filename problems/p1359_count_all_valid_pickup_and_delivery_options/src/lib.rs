pub struct Solution {}

impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        (1..n as usize).fold(1, |count, i| {
            count * (i * 2 + 1 + (i * 2 + 1) * i) % 1_000_000_007
        }) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1359() {
        assert_eq!(Solution::count_orders(500), 764678010);
        assert_eq!(Solution::count_orders(1), 1);
        assert_eq!(Solution::count_orders(2), 6);
        assert_eq!(Solution::count_orders(3), 90);
    }
}
