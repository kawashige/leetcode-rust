pub struct Solution {}

impl Solution {
    pub fn back_tracing(
        pairs: &Vec<Vec<i32>>,
        matched: &mut Vec<Vec<bool>>,
        remaining_games: &mut Vec<i32>,
        schedule: &mut Vec<Vec<i32>>,
    ) -> bool {
        if schedule.len() == pairs.len() {
            return true;
        }

        let mut unmatched_pairs = Vec::new();
        for i in 0..remaining_games.len() {
            if !schedule.is_empty() && schedule.last().unwrap().contains(&(i as i32)) {
                continue;
            }
            for j in 0..remaining_games.len() {
                if i == j
                    || (!schedule.is_empty() && schedule.last().unwrap().contains(&(j as i32)))
                    || matched[i][j]
                {
                    continue;
                }
                unmatched_pairs.push((remaining_games[i] + remaining_games[j], (i, j)));
            }
        }
        unmatched_pairs.sort_unstable();

        for i in (0..unmatched_pairs.len()).rev() {
            matched[unmatched_pairs[i].1.0][unmatched_pairs[i].1.1] = true;
            remaining_games[unmatched_pairs[i].1.0] -= 1;
            remaining_games[unmatched_pairs[i].1.1] -= 1;
            schedule.push(vec![
                unmatched_pairs[i].1.0 as i32,
                unmatched_pairs[i].1.1 as i32,
            ]);
            if Self::back_tracing(pairs, matched, remaining_games, schedule) {
                return true;
            }
            matched[unmatched_pairs[i].1.0][unmatched_pairs[i].1.1] = false;
            remaining_games[unmatched_pairs[i].1.0] += 1;
            remaining_games[unmatched_pairs[i].1.1] += 1;
            schedule.pop();
        }

        false
    }

    pub fn generate_schedule(n: i32) -> Vec<Vec<i32>> {
        if n < 5 {
            return Default::default();
        }

        let mut remaining_games = vec![n - 1; n as usize];
        let mut pairs = Vec::new();
        for i in 0..n as usize {
            for j in 0..n as usize {
                if i != j {
                    pairs.push(vec![i as i32, j as i32]);
                }
            }
        }

        let mut matched = vec![vec![false; pairs.len()]; pairs.len()];
        let mut schedule = Vec::new();

        if Self::back_tracing(&pairs, &mut matched, &mut remaining_games, &mut schedule) {
            schedule
        } else {
            Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3680() {
        assert_eq!(Solution::generate_schedule(3), vec![] as Vec<Vec<i32>>);
        assert_eq!(
            Solution::generate_schedule(5),
            vec![
                vec![0, 1],
                vec![2, 3],
                vec![0, 4],
                vec![1, 2],
                vec![3, 4],
                vec![0, 2],
                vec![1, 3],
                vec![2, 4],
                vec![0, 3],
                vec![1, 4],
                vec![2, 0],
                vec![3, 1],
                vec![4, 0],
                vec![2, 1],
                vec![4, 3],
                vec![1, 0],
                vec![3, 2],
                vec![4, 1],
                vec![3, 0],
                vec![4, 2]
            ]
        );
    }
}
fn main() {}
