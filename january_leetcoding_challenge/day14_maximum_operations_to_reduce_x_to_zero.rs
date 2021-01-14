pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let l = nums.len();
        let total_sum = nums.iter().sum::<i32>();
        let target = total_sum - x;

        if target == 0 {
            return l as i32;
        } else if target < 0 {
            return -1;
        }

        let mut sum = nums[0];
        let mut s = 0;
        let mut max = 0;
        for i in 1..nums.len() {
            sum += nums[i];
            while s < i && target < sum {
                sum -= nums[s];
                s += 1;
            }
            if sum == target {
                max = std::cmp::max(max, i - s + 1);
            }
        }
        if 0 < max {
            (l - max) as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day14() {
        assert_eq!(2, Solution::min_operations(vec![1, 1, 4, 2, 3], 5));
        assert_eq!(-1, Solution::min_operations(vec![5, 6, 7, 8, 9], 4));
        assert_eq!(5, Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10));
    }
}
