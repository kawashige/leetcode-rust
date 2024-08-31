pub struct Solution {}

impl Solution {
    pub fn max_strength(nums: Vec<i32>) -> i64 {
        if nums.len() == 1 {
            return nums[0] as i64;
        }
        let mut nums = nums;
        nums.sort_unstable();
        let count = nums.iter().fold([0; 3], |mut count, n| {
            if n < &0 {
                count[0] += 1;
            } else if n == &0 {
                count[1] += 1;
            } else {
                count[2] += 1;
            }
            count
        });
        if (0 < count[1] && (count[0] == 1 && count[2] == 0)) || (count[0] == 0 && count[2] == 0) {
            0
        } else {
            (0..(count[0] / 2) * 2).fold(1, |acc, i| acc * nums[i] as i64)
                * (count[0] + count[1]..nums.len()).fold(1, |acc, i| acc * nums[i] as i64)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2708() {
        assert_eq!(Solution::max_strength(vec![3, -1, -5, 2, 5, -9]), 1350);
        assert_eq!(Solution::max_strength(vec![-4, -5, -4]), 20);
    }
}
