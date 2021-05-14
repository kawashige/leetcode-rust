pub struct Solution {}

impl Solution {
    pub fn dfs(nums: &[i32], i: usize, sum: f64, num: i32, count: i32, k: i32, max: &mut f64) {
        if i == nums.len() {
            let tmp = sum + num as f64 / count as f64;
            if *max < tmp {
                *max = tmp;
            }
            return;
        }

        if k > 1 && count > 0 {
            Self::dfs(
                nums,
                i + 1,
                sum + num as f64 / count as f64,
                nums[i],
                1,
                k - 1,
                max,
            );
        }
        Self::dfs(nums, i + 1, sum, num + nums[i], count + 1, k, max);
    }

    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        let mut max = 0.0;
        Self::dfs(&nums, 1, 0.0, nums[0], 1, k, &mut max);
        max
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
