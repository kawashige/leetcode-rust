pub struct Solution {}

impl Solution {
    pub fn is_middle_element_unique(nums: Vec<i32>) -> bool {
        let middle = nums.len() / 2;
        (0..nums.len()).all(|i| i == middle || nums[i] != nums[middle])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3978() {
        assert!(Solution::is_middle_element_unique(vec![1, 2, 3]));
        assert!(!Solution::is_middle_element_unique(vec![1, 2, 2]));
    }
}

fn main() {}
