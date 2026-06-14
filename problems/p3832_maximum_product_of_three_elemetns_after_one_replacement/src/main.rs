pub struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        nums.sort_unstable_by_key(|num| -num.abs());
        nums[0].abs() as i64 * nums[1].abs() as i64 * 100_000
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3832() {
        assert_eq!(Solution::max_product(vec![-5, 7, 0]), 3500000);
        assert_eq!(Solution::max_product(vec![-4, -2, -1, -3]), 1200000);
        assert_eq!(Solution::max_product(vec![0, 10, 0]), 0);
    }
}

fn main() {}
