pub struct Solution {}

impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const M: usize = 1_000_000_007;

        let mut powers = Vec::new();
        let mut power = 1;
        for i in 0..32 {
            if n & 1 << i != 0 {
                powers.push(power);
            }
            power *= 2;
            power %= M;
        }

        queries
            .into_iter()
            .map(|q| {
                powers[q[0] as usize..=q[1] as usize]
                    .iter()
                    .fold(1, |acc, p| acc * p % M) as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2438() {
        assert_eq!(
            Solution::product_queries(15, vec![vec![0, 1], vec![2, 2], vec![0, 3]]),
            vec![2, 4, 64]
        );
        assert_eq!(Solution::product_queries(2, vec![vec![0, 0]]), vec![2]);
    }
}
