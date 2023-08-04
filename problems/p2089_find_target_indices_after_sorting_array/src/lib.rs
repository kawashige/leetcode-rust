pub struct Solution {}

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut values = vec![0; 101];
        for num in nums {
            values[num as usize] += 1;
        }
        let sum = (1..target).map(|i| values[i as usize]).sum::<i32>();
        (0..values[target as usize]).map(|i| i + sum).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2089() {
        assert_eq!(Solution::target_indices(vec![1, 2, 5, 2, 3], 2), vec![1, 2]);
        assert_eq!(Solution::target_indices(vec![1, 2, 5, 2, 3], 3), vec![3]);
        assert_eq!(Solution::target_indices(vec![1, 2, 5, 2, 3], 5), vec![4]);
    }
}
