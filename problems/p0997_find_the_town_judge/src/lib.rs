pub struct Solution {}

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let (trusted, trust) = trust.into_iter().fold(
            (vec![0; n as usize + 1], vec![false; n as usize + 1]),
            |(mut trusted, mut trust), t| {
                trusted[t[1] as usize] += 1;
                trust[t[0] as usize] = true;
                (trusted, trust)
            },
        );

        (1..=n)
            .find(|i| trusted[*i as usize] == n as usize - 1 && !trust[*i as usize])
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0997() {
        assert_eq!(Solution::find_judge(2, vec![vec![1, 2]]), 2);
        assert_eq!(Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
        assert_eq!(
            Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]),
            -1
        );
        assert_eq!(Solution::find_judge(3, vec![vec![1, 2], vec![2, 3]]), -1);
        assert_eq!(
            Solution::find_judge(
                4,
                vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]]
            ),
            3
        );
    }
}
