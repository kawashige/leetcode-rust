pub struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut results = vec![vec![1], vec![1, 1]];
        match num_rows {
            0 => vec![],
            1 => vec![vec![1]],
            2 => results,
            _ => {
                for i in 1..(num_rows as usize - 1) {
                    let mut row = vec![1];
                    for j in 0..(results[i].len() - 1) {
                        row.push(results[i][j] + results[i][j + 1]);
                    }
                    row.push(1);
                    results.push(row);
                }
                results
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0118() {
        assert_eq!(
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ],
            Solution::generate(5)
        );
    }
}
