pub struct Solution {}

impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        if n < k {
            return -1;
        }
        (1..=n)
            .filter(|i| n % i == 0)
            .nth(k as usize - 1)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day4() {
        assert_eq!(3, Solution::kth_factor(12, 3));
        assert_eq!(7, Solution::kth_factor(7, 2));
        assert_eq!(-1, Solution::kth_factor(4, 4));
        assert_eq!(1, Solution::kth_factor(1, 1));
        assert_eq!(4, Solution::kth_factor(1000, 3));
    }
}
