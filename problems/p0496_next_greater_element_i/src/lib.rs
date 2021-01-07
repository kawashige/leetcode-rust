pub struct Solution {}

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        nums1
            .into_iter()
            .map(|n1| {
                let p = (0..nums2.len()).find(|i| n1 == nums2[*i]).unwrap();
                *nums2[(p + 1)..].iter().find(|n2| &&n1 < n2).unwrap_or(&-1)
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0496() {
        assert_eq!(
            vec![-1, 3, -1],
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2])
        );
        assert_eq!(
            vec![3, -1],
            Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4])
        );
    }
}
