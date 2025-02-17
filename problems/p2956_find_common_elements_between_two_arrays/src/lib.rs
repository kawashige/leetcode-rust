pub struct Solution {}

impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        vec![
            nums1.iter().filter(|n| nums2.contains(n)).count() as i32,
            nums2.iter().filter(|n| nums1.contains(n)).count() as i32,
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2956() {
        assert_eq!(
            Solution::find_intersection_values(vec![2, 3, 2], vec![1, 2]),
            vec![2, 1]
        );
        assert_eq!(
            Solution::find_intersection_values(vec![4, 3, 2, 3, 1], vec![2, 2, 5, 2, 3, 6]),
            vec![3, 4]
        );
        assert_eq!(
            Solution::find_intersection_values(vec![3, 4, 2, 3], vec![1, 5]),
            vec![0, 0]
        );
    }
}
