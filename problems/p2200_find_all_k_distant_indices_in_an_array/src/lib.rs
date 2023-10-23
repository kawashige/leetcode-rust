pub struct Solution {}

impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let mut is_target = vec![false; nums.len()];
        for i in 0..nums.len() {
            if nums[i] == key {
                for j in
                    (i as i32 - k).max(0) as usize..((i as i32 + k + 1) as usize).min(nums.len())
                {
                    is_target[j] = true;
                }
            }
        }

        (0..nums.len() as i32)
            .filter(|i| is_target[*i as usize])
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2200() {
        assert_eq!(
            Solution::find_k_distant_indices(vec![3, 4, 9, 1, 3, 9, 5], 9, 1),
            vec![1, 2, 3, 4, 5, 6]
        );
        assert_eq!(
            Solution::find_k_distant_indices(vec![2, 2, 2, 2, 2], 2, 2),
            vec![0, 1, 2, 3, 4]
        )
    }
}
