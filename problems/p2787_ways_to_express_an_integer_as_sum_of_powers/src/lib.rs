pub struct Solution {}

impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        const M: i32 = 1_000_000_007;
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 1;

        for i in 1..=n {
            let p = i.pow(x as u32);
            if n < p {
                break;
            }
            for j in (0..=n - p).rev() {
                dp[(j + p) as usize] += dp[j as usize];
                dp[(j + p) as usize] %= M;
            }
        }

        dp[n as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2787() {
        assert_eq!(Solution::number_of_ways(10, 2), 1);
        assert_eq!(Solution::number_of_ways(4, 1), 2);
    }
}
