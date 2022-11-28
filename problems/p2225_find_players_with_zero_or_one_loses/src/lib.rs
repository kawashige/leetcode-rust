pub struct Solution {}

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (wins, loses) = matches.into_iter().fold(
            (vec![0; 100_001], vec![0; 100_001]),
            |(mut wins, mut loses), m| {
                wins[m[0] as usize] += 1;
                loses[m[1] as usize] += 1;
                (wins, loses)
            },
        );

        let mut anserws = vec![vec![]; 2];
        for i in 1..loses.len() {
            if loses[i] == 0 && 0 < wins[i] {
                anserws[0].push(i as i32);
            } else if loses[i] == 1 {
                anserws[1].push(i as i32);
            }
        }

        anserws
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2225() {
        assert_eq!(
            Solution::find_winners(vec![
                vec![1, 3],
                vec![2, 3],
                vec![3, 6],
                vec![5, 6],
                vec![5, 7],
                vec![4, 5],
                vec![4, 8],
                vec![4, 9],
                vec![10, 4],
                vec![10, 9]
            ]),
            vec![vec![1, 2, 10], vec![4, 5, 7, 8]]
        );
        assert_eq!(
            Solution::find_winners(vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]]),
            vec![vec![1, 2, 5, 6], vec![]]
        );
    }
}
