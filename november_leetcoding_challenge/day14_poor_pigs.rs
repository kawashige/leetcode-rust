pub struct Solution {}

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        if buckets < 2 {
            return 0;
        }
        let nums = (minutes_to_test / minutes_to_die) + 1;
        let mut current = nums;
        let mut result = 1;
        while current < buckets {
            current *= nums;
            result += 1;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day14() {
        assert_eq!(5, Solution::poor_pigs(1000, 15, 60));
        assert_eq!(2, Solution::poor_pigs(4, 15, 15));
        assert_eq!(0, Solution::poor_pigs(1, 1, 1));
    }
}
