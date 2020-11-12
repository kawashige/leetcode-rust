pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        nums1
            .into_iter()
            .collect::<HashSet<i32>>()
            .intersection(&nums2.into_iter().collect::<HashSet<i32>>())
            .into_iter()
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0349() {
        assert_eq!(
            vec![] as Vec<i32>,
            Solution::intersection(vec![1, 2, 2, 1], vec![3, 3])
        );
        assert_eq!(
            vec![2],
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2])
        );
        assert_eq!(
            vec![9, 4],
            Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4])
        );
    }
}
