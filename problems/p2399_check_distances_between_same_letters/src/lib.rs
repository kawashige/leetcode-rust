pub struct Solution {}

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let dist = s
            .as_bytes()
            .iter()
            .enumerate()
            .fold([100; 26], |mut dist, (i, b)| {
                if dist[(b - b'a') as usize] == 100 {
                    dist[(b - b'a') as usize] = i;
                } else {
                    dist[(b - b'a') as usize] = i - dist[(b - b'a') as usize] - 1;
                }
                dist
            });

        (0..distance.len()).all(|i| dist[i] == 100 || dist[i] as i32 == distance[i])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2399() {
        assert!(Solution::check_distances(
            "tffyjlpvhxpigmzysnwkquhxrtbosocqjzniwebvcelgukrm".to_string(),
            vec![
                15, 11, 9, 11, 3, 0, 30, 13, 23, 27, 25, 36, 33, 16, 1, 3, 10, 21, 11, 24, 22, 31,
                17, 13, 11, 18
            ]
        ));
        assert!(Solution::check_distances(
            "abaccb".to_string(),
            vec![1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ));
        assert!(!Solution::check_distances(
            "aa".to_string(),
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ));
    }
}
