pub struct Solution {}

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut present_in_num1 = vec![false; 2001];
        let mut present_in_num2 = vec![false; 2001];
        for i in 0..nums1.len() {
            present_in_num1[(nums1[i] + 1000) as usize] = true;
        }
        for i in 0..nums2.len() {
            present_in_num2[(nums2[i] + 1000) as usize] = true;
        }

        vec![
            (0..present_in_num1.len())
                .filter(|i| present_in_num1[*i] && !present_in_num2[*i])
                .map(|i| i as i32 - 1000)
                .collect(),
            (0..present_in_num2.len())
                .filter(|i| !present_in_num1[*i] && present_in_num2[*i])
                .map(|i| i as i32 - 1000)
                .collect(),
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2215() {
        assert_eq!(
            Solution::find_difference(vec![1, 2, 3], vec![2, 4, 6]),
            vec![vec![1, 3], vec![4, 6]]
        );
        assert_eq!(
            Solution::find_difference(vec![1, 2, 3, 3], vec![1, 1, 2, 2]),
            vec![vec![3], vec![]]
        );
    }
}
