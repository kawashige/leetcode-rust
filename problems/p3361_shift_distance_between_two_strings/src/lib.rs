pub struct Solution {}

impl Solution {
    pub fn shift_distance(
        s: String,
        t: String,
        next_cost: Vec<i32>,
        previous_cost: Vec<i32>,
    ) -> i64 {
        let mut cost = vec![vec![std::i64::MAX; 26]; 26];
        for i in 0..26 {
            for j in 0..26 {
                let mut k = i;
                let mut cost1 = 0;
                while k != j {
                    cost1 += next_cost[k] as i64;
                    k = (k + 1) % 26;
                }
                let mut k = i;
                let mut cost2 = 0;
                while k != j {
                    cost2 += previous_cost[k] as i64;
                    k = (k + 25) % 26;
                }
                cost[i][j] = cost1.min(cost2) as i64;
            }
        }

        (0..s.len())
            .map(|i| cost[(s.as_bytes()[i] - b'a') as usize][(t.as_bytes()[i] - b'a') as usize])
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3361() {
        assert_eq!(
            Solution::shift_distance(
                "abab".to_string(),
                "baba".to_string(),
                vec![
                    100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                ],
                vec![
                    1, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                ]
            ),
            2
        );
        assert_eq!(
            Solution::shift_distance(
                "leet".to_string(),
                "code".to_string(),
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
            ),
            31
        );
    }
}
