pub struct Solution {}

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        (nums.iter().max().unwrap() - nums.iter().min().unwrap()) as i64 * k as i64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3689() {
        assert_eq!(Solution::max_total_value(vec![1, 3, 2], 2), 4);
        assert_eq!(Solution::max_total_value(vec![4, 2, 5, 1], 3), 12);
    }
}

fn main() {}
