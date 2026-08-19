pub struct Solution {}

impl Solution {
    pub fn final_element(nums: Vec<i32>) -> i32 {
        nums[0].max(nums[nums.len() - 1])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3828() {
        assert_eq!(Solution::final_element(vec![1, 5, 2]), 2);
        assert_eq!(Solution::final_element(vec![3, 7]), 7);
    }
}

fn main() {}
