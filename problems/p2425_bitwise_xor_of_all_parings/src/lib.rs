pub struct Solution {}

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut result = 0;

        if nums2.len() % 2 == 1 {
            result = nums1.iter().fold(result, |acc, num| acc ^ num);
        }
        if nums1.len() % 2 == 1 {
            result = nums2.iter().fold(result, |acc, num| acc ^ num);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2425() {
        assert_eq!(Solution::xor_all_nums(vec![2, 1, 3], vec![10, 2, 5, 0]), 13);
        assert_eq!(Solution::xor_all_nums(vec![1, 2], vec![3, 4]), 0);
    }
}
