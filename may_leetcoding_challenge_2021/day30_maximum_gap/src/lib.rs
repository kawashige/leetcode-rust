pub struct Solution {}

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }

        let mut min = nums[0];
        let mut max = nums[0];
        for i in 1..nums.len() {
            min = std::cmp::min(min, nums[i]);
            max = std::cmp::max(max, nums[i]);
        }

        let delta = 1.max((max - min) / (nums.len() as i32 - 1));
        let len = ((max - min) / delta) as usize + 1;
        let mut mins = vec![None; len];
        let mut maxs = vec![None; len];

        for i in 0..nums.len() {
            if nums[i] == max || nums[i] == min {
                continue;
            }
            let j = ((nums[i] - min) / delta) as usize;
            if mins[j].is_none() {
                mins[j] = Some(nums[i]);
            } else {
                mins[j] = mins[j].map(|b_min| b_min.min(nums[i]));
            }
            if maxs[j].is_none() {
                maxs[j] = Some(nums[i]);
            } else {
                maxs[j] = maxs[j].map(|b_max| b_max.max(nums[i]));
            }
        }

        let mut prev = min;
        let mut max_gap = 0;

        for i in 0..mins.len() {
            if mins[i].is_none() {
                continue;
            }
            max_gap = std::cmp::max(max_gap, mins[i].unwrap() - prev);
            prev = maxs[i].unwrap();
        }
        std::cmp::max(max_gap, max - prev)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day30() {
        assert_eq!(Solution::maximum_gap(vec![1, 1, 1, 1]), 0);
        assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
        assert_eq!(Solution::maximum_gap(vec![10]), 0);
    }
}
