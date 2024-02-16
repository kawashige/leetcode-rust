pub struct Solution {}

impl Solution {
    pub fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32 {
        const M: usize = 1_000_000_007;

        if k < (end_pos - start_pos).abs() {
            return 0;
        }

        let mut ways = vec![vec![0; k as usize * 2 + 2]; k as usize + 1];
        ways[0][k as usize] = 1;

        for i in 1..=k as usize {
            for j in 0..ways[i].len() {
                if 0 < j {
                    ways[i][j] += ways[i - 1][j - 1];
                    ways[i][j] %= M;
                }
                if j < ways[i].len() - 1 {
                    ways[i][j] += ways[i - 1][j + 1];
                    ways[i][j] %= M;
                }
            }
        }

        ways.last().unwrap()[(end_pos - start_pos + k) as usize] as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2400() {
        // assert_eq!(Solution::number_of_ways(272, 270, 6), 1);
        assert_eq!(Solution::number_of_ways(1, 1000, 999), 1);
        assert_eq!(Solution::number_of_ways(1, 2, 3), 3);
        assert_eq!(Solution::number_of_ways(2, 5, 10), 0);
    }
}
