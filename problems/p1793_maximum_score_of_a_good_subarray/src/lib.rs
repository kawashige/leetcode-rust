pub struct Solution {}

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = vec![k as usize; nums[k as usize] as usize + 1];
        let mut min = nums[k as usize];
        for i in (0..=k as usize).rev() {
            min = min.min(nums[i]);
            left[min as usize] = i;
        }
        let mut right = vec![k as usize; nums[k as usize] as usize + 1];
        let mut min = nums[k as usize];
        for i in k as usize + 1..nums.len() {
            min = min.min(nums[i]);
            right[min as usize] = i;
        }
        for i in (0..left.len() - 1).rev() {
            left[i] = left[i].min(left[i + 1]);
            right[i] = right[i].max(right[i + 1]);
        }

        (0..left.len())
            .map(|i| (i * (right[i] - left[i] + 1)) as i32)
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1793() {
        assert_eq!(Solution::maximum_score(vec![1, 4, 3, 7, 4, 5], 3), 15);
        assert_eq!(Solution::maximum_score(vec![5, 5, 4, 5, 4, 1, 1, 1], 0), 20);
    }
}
