pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut dp = vec![std::i32::MAX; n + 1];
        dp[0] = 0;

        for i in 0..n {
            for j in 1..4 {
                if n < i + j {
                    break;
                }
                dp[i + j] = dp[i + j].min(dp[i] + costs[i + j - 1] + j as i32 * j as i32);
            }
        }

        dp[n]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3693() {
        assert_eq!(Solution::climb_stairs(4, vec![1, 2, 3, 4]), 13);
        assert_eq!(Solution::climb_stairs(4, vec![5, 1, 6, 2]), 11);
        assert_eq!(Solution::climb_stairs(3, vec![9, 8, 3]), 12);
    }
}

fn main() {}
