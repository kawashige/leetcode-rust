pub struct Solution {}

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let mut matrix = vec![vec![false; n as usize]; languages.len()];
        for i in 0..languages.len() {
            for l in &languages[i] {
                matrix[i][*l as usize - 1] = true;
            }
        }

        let mut has_cannot_talk = vec![false; languages.len()];
        for f in &friendships {
            if languages[f[0] as usize - 1]
                .iter()
                .all(|l| !matrix[f[1] as usize - 1][*l as usize - 1])
            {
                has_cannot_talk[f[0] as usize - 1] = true;
                has_cannot_talk[f[1] as usize - 1] = true;
            }
        }

        let mut max = friendships.len() as i32;

        for l in 0..n as usize {
            let mut count = 0;
            for p in 0..has_cannot_talk.len() {
                if has_cannot_talk[p] && !matrix[p][l] {
                    count += 1;
                }
            }
            max = max.min(count);
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1733() {
        assert_eq!(
            Solution::minimum_teachings(
                2,
                vec![vec![1], vec![2], vec![1, 2]],
                vec![vec![1, 2], vec![1, 3], vec![2, 3]]
            ),
            1
        );
        assert_eq!(
            Solution::minimum_teachings(
                3,
                vec![vec![2], vec![1, 3], vec![1, 2], vec![3]],
                vec![vec![1, 4], vec![1, 2], vec![3, 4], vec![2, 3]]
            ),
            2
        );
    }
}
