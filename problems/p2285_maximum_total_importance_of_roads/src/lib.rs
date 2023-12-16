pub struct Solution {}

impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut count = vec![0; n as usize];
        for road in roads {
            count[road[0] as usize] += 1;
            count[road[1] as usize] += 1;
        }
        count.sort_unstable();
        count
            .into_iter()
            .zip(1..)
            .map(|(c, i)| c as i64 * i as i64)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2285() {
        assert_eq!(
            Solution::maximum_importance(
                5,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![1, 3],
                    vec![2, 4]
                ]
            ),
            43
        );
        assert_eq!(
            Solution::maximum_importance(5, vec![vec![0, 3], vec![2, 4], vec![1, 3]]),
            20
        );
    }
}
