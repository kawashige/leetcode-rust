pub struct Solution {}

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.sort();
        nums2.sort();
        let mut results = Vec::new();
        let mut i = 0;
        let mut j = 0;
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] == nums2[j] {
                results.push(nums1[i]);
                i += 1;
                j += 1;
            } else if nums1[i] < nums2[j] {
                i += 1;
            } else {
                j += 1;
            }
        }
        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0350() {
        assert_eq!(
            vec![] as Vec<i32>,
            Solution::intersect(vec![1, 2, 2, 1], vec![3, 3])
        );
        assert_eq!(
            vec![2, 2],
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2])
        );
        assert_eq!(
            vec![4, 9],
            Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4])
        );
    }
}
