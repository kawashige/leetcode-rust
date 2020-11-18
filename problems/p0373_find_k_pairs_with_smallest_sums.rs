pub struct Solution {}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        if k < 1 || nums1.is_empty() || nums2.is_empty() {
            return Vec::new();
        }

        let mut results = Vec::new();

        let mut pairs: Vec<i32> = vec![-1; nums2.len()];
        let mut j = 0;
        while results.len() < k as usize && j < pairs.len() {
            let mut min = std::i32::MAX;
            let mut min_index = 0;
            for i in j..pairs.len() {
                let sum = nums1[(pairs[i] + 1) as usize] + nums2[i];
                if sum < min {
                    min = sum;
                    min_index = i;
                }
                if pairs[i] == -1 {
                    break;
                }
            }
            results.push(vec![
                nums1[(pairs[min_index] + 1) as usize],
                nums2[min_index],
            ]);
            pairs[min_index] += 1;
            if nums1.len() - 1 <= pairs[min_index] as usize {
                j = min_index + 1;
            }
        }

        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0373() {
        assert_eq!(
            vec![vec![1, 2], vec![1, 4], vec![1, 6]],
            Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3)
        );
        assert_eq!(
            vec![vec![1, 1], vec![1, 1]],
            Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2)
        );
        assert_eq!(
            vec![vec![1, 3], vec![2, 3]],
            Solution::k_smallest_pairs(vec![1, 2], vec![3], 3)
        );
    }
}
