pub struct Solution {}

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut counts = vec![vec![false; 101]; 3];
        let nums = [nums1, nums2, nums3];
        for i in 0..nums.len() {
            for j in 0..nums[i].len() {
                counts[i][nums[i][j] as usize] = true;
            }
        }
        println!("{:?}", counts);
        (0..counts[0].len())
            .filter_map(|i| {
                if 1 < counts.iter().filter(|c| c[i]).count() {
                    Some(i as i32)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2032() {
        assert_eq!(
            Solution::two_out_of_three(vec![1, 1, 3, 2], vec![2, 3], vec![3]),
            vec![2, 3]
        );
        assert_eq!(
            Solution::two_out_of_three(vec![3, 1], vec![2, 3], vec![1, 2]),
            vec![1, 2, 3]
        );
        assert_eq!(
            Solution::two_out_of_three(vec![1, 2, 2], vec![4, 3, 3], vec![5]),
            vec![]
        );
    }
}
