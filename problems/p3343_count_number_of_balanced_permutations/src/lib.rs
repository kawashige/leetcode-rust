pub struct Solution {}

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn count_balanced_permutations(num: String) -> i32 {
        let (count, sum) = num
            .as_bytes()
            .iter()
            .fold(([0; 10], 0), |(mut count, sum), b| {
                count[(b - b'0') as usize] += 1;
                (count, sum + (b - b'0') as usize)
            });
        let n = num.len();
        let max_odd = (n + 1) / 2;

        if sum % 2 == 1 {
            return 0;
        }
        let target = sum / 2;
        let mut comb = vec![vec![0; max_odd as usize + 1]; max_odd as usize + 1];

        for i in 0..=max_odd {
            comb[i][i] = 1;
            comb[i][0] = 1;
            for j in 1..i {
                comb[i][j] = (comb[i - 1][j] + comb[i - 1][j - 1]) % MOD;
            }
        }
        let mut dp = vec![vec![0; max_odd + 1]; target + 1];
        dp[0][0] = 1;

        let mut tmp_count = 0;
        let mut tmp_sum = 0;

        for i in 0..10 {
            tmp_count += count[i];
            tmp_sum += i * count[i];

            for odd in (0.max(tmp_count as i32 - (n - max_odd) as i32) as usize
                ..=max_odd.min(tmp_count))
                .rev()
            {
                let even = tmp_count - odd;
                for curr in
                    (0.max(tmp_sum as i32 - target as i32) as usize..=tmp_sum.min(target)).rev()
                {
                    let mut res = 0;
                    for j in
                        (0.max(count[i] as i32 - even as i32) as usize..=count[i].min(odd)).rev()
                    {
                        if i * j <= curr {
                            let ways = (comb[odd][j] * comb[even][count[i] - j]) % MOD;
                            res = (res + ways * dp[curr - i * j][odd - j]) % MOD;
                        }
                    }
                    dp[curr][odd] = res % MOD;
                }
            }
        }

        dp[target][max_odd] as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3343() {
        assert_eq!(Solution::count_balanced_permutations("123".to_string()), 2);
        assert_eq!(Solution::count_balanced_permutations("112".to_string()), 1);
        assert_eq!(
            Solution::count_balanced_permutations("12345".to_string()),
            0
        );
    }
}
