pub struct Solution {}

impl Solution {
    pub fn alternating_sum(nums: Vec<i32>) -> i32 {
        (0..nums.len())
            .map(|i| if i % 2 == 0 { 1 } else { -1 } * nums[i])
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3701() {
        assert_eq!(Solution::alternating_sum(vec![1, 3, 5, 7]), -4);
        assert_eq!(Solution::alternating_sum(vec![100]), 100);
    }
}

fn main() {}
