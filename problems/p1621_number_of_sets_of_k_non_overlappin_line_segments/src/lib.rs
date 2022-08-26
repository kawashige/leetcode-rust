
pub struct Solution {}

impl Solution {
    const M: usize = 1_000_000_007;

    pub fn recurse(n: usize, k: usize, memo: &mut Vec<Vec<usize>>) -> usize {
        if k == 0 {
            return 1;
        }

        if memo[n][k] < Self::M {
            return memo[n][k];
        }

        let mut count = 0;
        for i in 1..n - (k - 1) {
            count = (count + i * Self::recurse(n - i, k - 1, memo) % Self::M) % Self::M;
        }

        memo[n][k] = count;
        count
    }

    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        let mut memo = vec![vec![Self::M; k as usize + 1]; n as usize + 1];
        Self::recurse(n as usize, k as usize, &mut memo) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1621() {
        assert_eq!(Solution::number_of_sets(751, 201), 88243036);
        assert_eq!(Solution::number_of_sets(4, 2), 5);
        assert_eq!(Solution::number_of_sets(3, 1), 3);
        assert_eq!(Solution::number_of_sets(30, 7), 796297179);
    }
}
