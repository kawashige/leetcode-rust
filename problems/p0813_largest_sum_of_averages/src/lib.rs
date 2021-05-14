pub struct Solution {}

impl Solution {
    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        let n = nums.len();
        let p = std::iter::once(0.0)
            .chain(nums.into_iter().scan(0.0, |sum, num| {
                *sum += num as f64;
                Some(*sum)
            }))
            .collect::<Vec<f64>>();

        let mut dp = vec![0.0; n];
        for i in 0..n {
            dp[i] = (p[n] - p[i]) / (n - i) as f64;
        }

        for _ in 0..(k - 1) {
            for i in 0..n {
                for j in (i + 1)..n {
                    let r = (p[j] - p[i]) / (j - i) as f64 + dp[j];
                    if dp[i] < r {
                        dp[i] = r;
                    }
                }
            }
        }

        dp[0]
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
