pub struct Solution {}

impl Solution {
    const MOD: i32 = 1_000_000_007;

    pub fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
        let m = *nums.iter().max().unwrap() as usize;

        let mut dp = vec![vec![0; m + 1]; m + 1];
        dp[0][0] = 1;

        for &num in &nums {
            let num = num as usize;
            let mut ndp = vec![vec![0; m + 1]; m + 1];

            for j in 0..=m {
                let divisor1 = Self::gcd(j as i32, num as i32) as usize;
                for k in 0..=m {
                    let val = dp[j][k];
                    if val == 0 {
                        continue;
                    }

                    let divisor2 = Self::gcd(k as i32, num as i32) as usize;
                    ndp[j][k] = (ndp[j][k] + val) % Self::MOD;
                    ndp[divisor1][k] = (ndp[divisor1][k] + val) % Self::MOD;
                    ndp[j][divisor2] = (ndp[j][divisor2] + val) % Self::MOD;
                }
            }
            dp = ndp;
        }

        let mut ans = 0;
        for j in 1..=m {
            ans = (ans + dp[j][j]) % Self::MOD;
        }

        ans
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = a;
            a = b;
            b = temp % b;
        }
        a
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3336() {
        assert_eq!(Solution::subsequence_pair_count(vec![1, 2, 3, 4]), 10);
        assert_eq!(Solution::subsequence_pair_count(vec![10, 20, 30]), 2);
        assert_eq!(Solution::subsequence_pair_count(vec![1, 1, 1, 1]), 50);
    }
}

fn main() {}
