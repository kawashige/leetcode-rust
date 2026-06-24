pub struct Solution {}

impl Solution {
    const MOD: i32 = 1_000_000_007;

    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        let m = (r - l + 1) as usize;

        let mut dp0 = vec![1; m];
        let mut dp1 = vec![1; m];
        let mut sum0 = vec![0; m + 1];
        let mut sum1 = vec![0; m + 1];

        for _ in 1..n {
            for j in 0..m {
                sum0[j + 1] = (sum0[j] + dp0[j]) % Self::MOD;
                sum1[j + 1] = (sum1[j] + dp1[j]) % Self::MOD;
            }

            for j in 0..m {
                dp0[j] = sum1[j];
                dp1[j] = (sum0[m] - sum0[j + 1] + Self::MOD) % Self::MOD;
            }
        }

        let ans0 = dp0.iter().fold(0, |acc, &x| (acc + x) % Self::MOD);
        let ans1 = dp1.iter().fold(0, |acc, &x| (acc + x) % Self::MOD);

        (ans0 + ans1) % Self::MOD
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3699() {
        assert_eq!(Solution::zig_zag_arrays(3, 4, 5), 2);
        assert_eq!(Solution::zig_zag_arrays(3, 1, 3), 10);
    }
}

fn main() {}
