pub struct Solution {}

impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let mut count = vec![vec![0; 11]; n as usize];
        for p in pick {
            count[p[0] as usize][p[1] as usize] += 1;
        }
        (0..count.len())
            .filter(|p| p < &count[*p].iter().max().unwrap())
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3238() {
        assert_eq!(
            Solution::winning_player_count(
                4,
                vec![
                    vec![0, 0],
                    vec![1, 0],
                    vec![1, 0],
                    vec![2, 1],
                    vec![2, 1],
                    vec![2, 0]
                ]
            ),
            2
        );
        assert_eq!(
            Solution::winning_player_count(5, vec![vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4]]),
            0
        );
        assert_eq!(
            Solution::winning_player_count(5, vec![vec![1, 1], vec![2, 4], vec![2, 4], vec![2, 4]]),
            1
        );
    }
}
