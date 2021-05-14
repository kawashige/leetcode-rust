use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn dfs(nums: &[i32], k: usize, memo: &mut HashMap<(usize, usize), f64>) -> f64 {
        if nums.len() == k {
            return nums.iter().sum::<i32>() as f64;
        } else if k == 1 {
            return nums.iter().sum::<i32>() as f64 / nums.len() as f64;
        }

        if let Some(c) = memo.get(&(nums.len(), k)) {
            return *c;
        }

        let mut num = 0;
        let mut max = 0.0;
        for j in 0..(nums.len() + 1 - k) {
            num += nums[j];
            let r = num as f64 / (j + 1) as f64 + Self::dfs(&nums[(j + 1)..], k - 1, memo);
            if r > max {
                max = r;
            }
        }

        memo.insert((nums.len(), k), max);
        max
    }

    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        Self::dfs(&nums, k as usize, &mut HashMap::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0813() {
        assert_eq!(
            Solution::largest_sum_of_averages(
                vec![
                    9183, 4389, 9284, 9237, 6853, 1630, 7881, 8356, 8046, 2738, 5834, 7916, 8260,
                    3033, 9773, 6566, 3629, 9581, 961, 2971
                ],
                10
            ),
            74020.46666666667
        );
        assert_eq!(
            Solution::largest_sum_of_averages(vec![9, 1, 2, 3, 9], 3),
            20.0
        );
    }
}
