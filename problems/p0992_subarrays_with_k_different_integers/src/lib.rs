pub struct Solution {}

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;

        for i in 0..nums.len() {
            let mut count = 0;
            let mut counts = vec![0; nums.len() as usize + 1];
            let mut j = i;

            while count <= k && j < nums.len() {
                counts[nums[j] as usize] += 1;
                if counts[nums[j] as usize] == 1 {
                    count += 1;
                }
                if count == k {
                    result += 1;
                }
                j += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0992() {
        assert_eq!(
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2),
            7
        );
        assert_eq!(
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3),
            3
        );
    }
}
