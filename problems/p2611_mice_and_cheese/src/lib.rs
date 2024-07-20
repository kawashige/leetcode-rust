pub struct Solution {}

impl Solution {
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let mut diff = (0..reward1.len())
            .map(|i| reward1[i] - reward2[i])
            .collect::<Vec<_>>();
        diff.sort_unstable();
        reward2.into_iter().sum::<i32>() + diff.into_iter().rev().take(k as usize).sum::<i32>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2611() {
        assert_eq!(
            Solution::mice_and_cheese(vec![1, 1, 3, 4], vec![4, 4, 1, 1], 2),
            15
        );
        assert_eq!(Solution::mice_and_cheese(vec![1, 1], vec![1, 1], 2), 2);
    }
}
