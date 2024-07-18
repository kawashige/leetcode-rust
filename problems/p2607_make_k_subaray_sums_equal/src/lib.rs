pub struct Solution {}

impl Solution {
    pub fn make_sub_k_sum_equal(arr: Vec<i32>, k: i32) -> i64 {
        let mut seen = vec![false; arr.len()];
        let mut operations = 0;

        for i in 0..arr.len() {
            if seen[i] {
                continue;
            }
            let mut nums = Vec::with_capacity(arr.len());
            let mut j = i;
            while !seen[j] {
                nums.push(arr[j]);
                seen[j] = true;
                j = (j + k as usize) % arr.len();
            }
            nums.sort_unstable();
            let mid = nums[nums.len() / 2];
            operations += nums
                .into_iter()
                .map(|num| (num - mid).abs() as i64)
                .sum::<i64>()
        }

        operations
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2607() {
        assert_eq!(Solution::make_sub_k_sum_equal(vec![1, 4, 1, 3], 2), 1);
        assert_eq!(Solution::make_sub_k_sum_equal(vec![2, 5, 5, 7], 3), 5);
    }
}
