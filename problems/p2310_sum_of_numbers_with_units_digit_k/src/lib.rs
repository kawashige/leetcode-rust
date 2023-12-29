pub struct Solution {}

impl Solution {
    pub fn minimum_numbers(num: i32, k: i32) -> i32 {
        if num == 0 {
            return 0;
        }
        (1..=num / k.max(1))
            .find(|i| (num - k * i) % 10 == 0)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2310() {
        assert_eq!(Solution::minimum_numbers(58, 9), 2);
        assert_eq!(Solution::minimum_numbers(37, 2), -1);
        assert_eq!(Solution::minimum_numbers(0, 7), 0);
    }
}
