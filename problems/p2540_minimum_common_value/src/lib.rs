pub struct Solution {}

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i1 = 0;
        let mut i2 = 0;

        loop {
            while i1 < nums1.len() && nums1[i1] < nums2[i2] {
                i1 += 1;
            }
            if nums1.len() <= i1 {
                break;
            }
            while i2 < nums2.len() && nums1[i1] > nums2[i2] {
                i2 += 1;
            }
            if nums2.len() <= i2 {
                break;
            }
            if nums1[i1] == nums2[i2] {
                return nums1[i1];
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2540() {
        assert_eq!(Solution::get_common(vec![1, 2, 3], vec![2, 4]), 2);
        assert_eq!(Solution::get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]), 2);
    }
}
