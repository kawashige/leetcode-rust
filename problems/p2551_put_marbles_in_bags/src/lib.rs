pub struct Solution {}

impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let mut inside = (0..weights.len() - 1)
            .map(|i| weights[i] as i64 + weights[i + 1] as i64)
            .collect::<Vec<_>>();
        inside.sort_unstable();
        println!("{:?}", inside);
        inside[inside.len() + 1 - k as usize..].iter().sum::<i64>()
            - inside[0..k as usize - 1].iter().sum::<i64>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2551() {
        assert_eq!(Solution::put_marbles(vec![1, 3, 5, 1], 2), 4);
        assert_eq!(Solution::put_marbles(vec![1, 3], 2), 0);
    }
}
