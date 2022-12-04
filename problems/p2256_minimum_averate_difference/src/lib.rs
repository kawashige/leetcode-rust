pub struct Solution {}

impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let sum: usize = nums.iter().map(|num| *num as usize).sum();
        let mut cur_sum = 0;
        let mut min_diff = std::i32::MAX;
        let mut min_i = 0;

        for i in 0..nums.len() - 1 {
            cur_sum += nums[i] as usize;
            let cur_diff = ((cur_sum / (i + 1)) as i32
                - ((sum - cur_sum) / (nums.len() - i - 1)) as i32)
                .abs();
            if cur_diff < min_diff {
                min_diff = cur_diff;
                min_i = i;
            }
        }

        if min_diff <= (sum / nums.len()) as i32 {
            min_i as i32
        } else {
            nums.len() as i32 - 1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2256() {
        assert_eq!(
            Solution::minimum_average_difference(vec![2, 5, 3, 9, 5, 3]),
            3
        );
        assert_eq!(Solution::minimum_average_difference(vec![0]), 0);
    }
}
