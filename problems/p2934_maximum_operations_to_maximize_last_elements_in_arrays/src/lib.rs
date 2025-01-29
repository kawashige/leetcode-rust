pub struct Solution {}

impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        if nums1[nums1.len() - 1] == nums2[nums2.len() - 1] {
            if nums1.iter().max().unwrap() == nums1.last().unwrap()
                && nums2.iter().max().unwrap() == nums2.last().unwrap()
            {
                0
            } else {
                -1
            }
        } else {
            let min = nums1.last().unwrap().min(nums2.last().unwrap());
            let max = nums1.last().unwrap().max(nums2.last().unwrap());
            if nums1.iter().any(|n| max < n)
                || nums2.iter().any(|n| max < n)
                || (0..nums1.len()).any(|i| min < &nums1[i] && min < &nums2[i])
            {
                return -1;
            }
            let c1 = nums1.iter().filter(|n| min < n).count();
            let c2 = nums2.iter().filter(|n| min < n).count();
            c1.min(c2) as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2934() {
        assert_eq!(Solution::min_operations(vec![1, 2, 7], vec![4, 5, 3]), 1);
        assert_eq!(
            Solution::min_operations(vec![2, 3, 4, 5, 9], vec![8, 8, 4, 4, 4]),
            2
        );
        assert_eq!(Solution::min_operations(vec![1, 5, 4], vec![2, 5, 3]), -1);
    }
}
