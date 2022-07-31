use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut count = 0;

        let mut multiple1 = HashMap::new();
        for i in 0..nums1.len() {
            for j in (i + 1)..nums1.len() {
                *multiple1
                    .entry(nums1[i] as usize * nums1[j] as usize)
                    .or_insert(0) += 1;
            }
        }
        for i in 0..nums2.len() {
            count += multiple1
                .get(&(nums2[i] as usize * nums2[i] as usize))
                .unwrap_or(&0);
        }

        let mut multiple2 = HashMap::new();
        for i in 0..nums2.len() {
            for j in (i + 1)..nums2.len() {
                *multiple2
                    .entry(nums2[i] as usize * nums2[j] as usize)
                    .or_insert(0) += 1;
            }
        }
        for i in 0..nums1.len() {
            count += multiple2
                .get(&(nums1[i] as usize * nums1[i] as usize))
                .unwrap_or(&0);
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1577() {
        assert_eq!(Solution::num_triplets(vec![43024, 99908], vec![1864]), 0);
        assert_eq!(Solution::num_triplets(vec![7, 4], vec![5, 2, 8, 9]), 1);
        assert_eq!(Solution::num_triplets(vec![1, 1], vec![1, 1, 1]), 9);
        assert_eq!(
            Solution::num_triplets(vec![7, 7, 8, 3], vec![1, 2, 9, 7]),
            2
        );
    }
}
