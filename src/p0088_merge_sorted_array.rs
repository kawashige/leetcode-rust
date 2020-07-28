pub struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m as usize;
        let mut j = n as usize;
        let mut k = nums1.len();

        while j > 0 {
            if i == 0 {
                nums1[k - 1] = nums2[j - 1];
                k -= 1;
                j -= 1;
            } else if nums1[i - 1] > nums2[j - 1] {
                nums1[k - 1] = nums1[i - 1];
                k -= 1;
                i -= 1;
            } else {
                nums1[k - 1] = nums2[j - 1];
                k -= 1;
                j -= 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_88() {
        let mut nums1 = vec![4, 5, 6, 0, 0, 0];
        let mut nums2 = vec![1, 2, 3];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], nums1);
    }
}
