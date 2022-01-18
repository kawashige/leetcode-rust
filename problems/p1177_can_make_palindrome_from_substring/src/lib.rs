pub struct Solution {}

impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut counts = vec![vec![0; 26]; s.len() + 1];
        for (i, c) in s.as_bytes().iter().enumerate() {
            counts[i + 1][*c as usize - 0x61] += 1;
            for j in 0..26 {
                counts[i + 1][j] += counts[i][j];
            }
        }

        queries
            .into_iter()
            .map(|query| {
                let remains = (0..26)
                    .map(|i| (counts[query[1] as usize + 1][i] - counts[query[0] as usize][i]) % 2)
                    .sum::<i32>()
                    - query[2] * 2;
                remains <= (query[1] - query[0] + 1) % 2
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1177() {
        assert_eq!(
            Solution::can_make_pali_queries(
                "abcda".to_string(),
                vec![
                    vec![3, 3, 0],
                    vec![1, 2, 0],
                    vec![0, 3, 1],
                    vec![0, 3, 2],
                    vec![0, 4, 1]
                ]
            ),
            vec![true, false, false, true, true]
        );
        assert_eq!(
            Solution::can_make_pali_queries("lyb".to_string(), vec![vec![0, 1, 0], vec![2, 2, 1]]),
            vec![false, true]
        );
    }
}
