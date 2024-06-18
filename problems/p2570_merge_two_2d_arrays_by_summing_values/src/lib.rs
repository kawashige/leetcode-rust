pub struct Solution {}

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut values = vec![0; 1001];
        for i in 0..nums1.len() {
            values[nums1[i][0] as usize] += nums1[i][1];
        }
        for i in 0..nums2.len() {
            values[nums2[i][0] as usize] += nums2[i][1];
        }
        (0..values.len())
            .filter_map(|i| {
                if values[i] == 0 {
                    None
                } else {
                    Some(vec![i as i32, values[i]])
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2570() {
        assert_eq!(
            Solution::merge_arrays(
                vec![vec![1, 2], vec![2, 3], vec![4, 5]],
                vec![vec![1, 4], vec![3, 2], vec![4, 1]]
            ),
            vec![vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]]
        );
        assert_eq!(
            Solution::merge_arrays(
                vec![vec![2, 4], vec![3, 6], vec![5, 5]],
                vec![vec![1, 3], vec![4, 3]]
            ),
            vec![vec![1, 3], vec![2, 4], vec![3, 6], vec![4, 3], vec![5, 5]]
        );
    }
}
