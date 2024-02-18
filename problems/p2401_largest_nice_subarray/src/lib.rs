pub struct Solution {}

impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut max_indices = vec![-1; 32];
        let mut result = 0;
        let mut tmp_max = 0;

        for i in 0..nums.len() {
            let mut max = tmp_max;
            for j in 0..32 {
                if nums[i] & 1 << j != 0 {
                    max = max.max(max_indices[j] + 1);
                    max_indices[j] = i as i32;
                }
            }
            tmp_max = max;
            result = result.max(i as i32 - max + 1);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2401() {
        assert_eq!(
            Solution::longest_nice_subarray(vec![
                744437702, 379056602, 145555074, 392756761, 560864007, 934981918, 113312475, 1090,
                16384, 33, 217313281, 117883195, 978927664
            ]),
            3
        );
        assert_eq!(Solution::longest_nice_subarray(vec![1, 3, 8, 48, 10]), 3);
        assert_eq!(Solution::longest_nice_subarray(vec![3, 1, 5, 11, 13]), 1);
    }
}
