pub struct Solution {}

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let mut ones_count = vec![0; nums.len()];
        ones_count[0] = nums[0] as usize;
        for i in 1..nums.len() {
            ones_count[i] = ones_count[i - 1] + nums[i] as usize;
        }

        let ones = *ones_count.last().unwrap();
        if ones == 0 {
            return 0;
        }

        let mut min = nums.len() as i32;

        for i in 0..nums.len() {
            let swap = ones
                - if nums.len() <= i + ones - 1 {
                    ones - ones_count[i - 1] + ones_count[ones - (nums.len() - i) - 1]
                } else {
                    ones_count[i + ones - 1] - if i == 0 { 0 } else { ones_count[i - 1] }
                };
            min = min.min(swap as i32);
        }

        min
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2134() {
        assert_eq!(Solution::min_swaps(vec![0, 1, 0, 1, 1, 0, 0]), 1);
        assert_eq!(Solution::min_swaps(vec![0, 1, 1, 1, 0, 0, 1, 1, 0]), 2);
        assert_eq!(Solution::min_swaps(vec![1, 1, 0, 0, 1]), 0);
    }
}
