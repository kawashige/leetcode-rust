pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        let mut result = sorted_nums[0] + sorted_nums[1] + sorted_nums[2];
        let mut result_diff = (target - result).abs();
        let mut i = 0;
        while i < sorted_nums.len() {
            let mut j = i + 1;
            while j < sorted_nums.len() {
                let sum_2 = sorted_nums[i] + sorted_nums[j];
                for k in (j + 1)..sorted_nums.len() {
                    let sum = sum_2 + sorted_nums[k];
                    let diff = (target - sum).abs();
                    if diff == 0 {
                        return target;
                    } else if diff < result_diff {
                        result = sum;
                        result_diff = diff;
                    }
                }
                j += 1;
                while j < sorted_nums.len() && sorted_nums[j - 1] == sorted_nums[j] {
                    j += 1;
                }
            }
            i += 1;
            while i < sorted_nums.len() && sorted_nums[i - 1] == sorted_nums[i] {
                i += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0016() {
        assert_eq!(2, Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
    }
}
