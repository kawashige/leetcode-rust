pub struct Solution {}

impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut j = nums2.len() - 1;
        for i in (0..nums1.len()).rev() {
            while 0 < j && nums2[j] < nums1[i] {
                j -= 1;
            }
            if nums1[i] <= nums2[j] {
                max = max.max(j as i32 - i as i32);
            }
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1855() {
        assert_eq!(
            Solution::max_distance(vec![55, 30, 5, 4, 2], vec![100, 20, 10, 10, 5]),
            2
        );
        assert_eq!(Solution::max_distance(vec![2, 2, 2], vec![10, 10, 1]), 1);
        assert_eq!(
            Solution::max_distance(vec![30, 29, 19, 5], vec![25, 25, 25, 25, 25]),
            2
        );
    }
}
