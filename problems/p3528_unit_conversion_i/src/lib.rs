pub struct Solution {}

impl Solution {
    pub fn base_unit_conversions(conversions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut list = vec![vec![]; conversions.len() + 1];
        for i in 0..conversions.len() {
            list[conversions[i][0] as usize]
                .push((conversions[i][1] as usize, conversions[i][2] as i64));
        }

        let mut stack = vec![(0, 1)];
        let mut result = vec![std::i32::MAX; conversions.len() + 1];

        while let Some((n, u)) = stack.pop() {
            result[n] = u as i32;
            for next in &list[n] {
                stack.push((next.0, (u * next.1) % 1_000_000_007));
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3528() {
        assert_eq!(
            Solution::base_unit_conversions(vec![vec![0, 1, 2], vec![1, 2, 3]]),
            vec![1, 2, 6]
        );
        assert_eq!(
            Solution::base_unit_conversions(vec![
                vec![0, 1, 2],
                vec![0, 2, 3],
                vec![1, 3, 4],
                vec![1, 4, 5],
                vec![2, 5, 2],
                vec![4, 6, 3],
                vec![5, 7, 4]
            ]),
            vec![1, 2, 3, 8, 10, 6, 30, 24]
        );
    }
}
