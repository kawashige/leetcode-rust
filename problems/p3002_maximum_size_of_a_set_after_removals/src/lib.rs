use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn maximum_set_size(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n1 = nums1.len() / 2;
        let n2 = nums2.len() / 2;
        let set1: HashSet<&i32> = HashSet::from_iter(nums1.iter());
        let set2 = HashSet::from_iter(nums2.iter());
        let c = set1.intersection(&set2).count();

        ((set1.len() - c).min(n1)
            + (set2.len() - c).min(n2)
            + ((n1 - (set1.len() - c).min(n1)) + (n2 - (set2.len() - c).min(n2))).min(c))
            as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3002() {
        assert_eq!(
            Solution::maximum_set_size(vec![1, 2, 1, 2], vec![1, 1, 1, 1]),
            2
        );
        assert_eq!(
            Solution::maximum_set_size(vec![1, 2, 3, 4, 5, 6], vec![2, 3, 2, 3, 2, 3]),
            5
        );
        assert_eq!(
            Solution::maximum_set_size(vec![1, 1, 2, 2, 3, 3], vec![4, 4, 5, 5, 6, 6]),
            6
        );
    }
}
