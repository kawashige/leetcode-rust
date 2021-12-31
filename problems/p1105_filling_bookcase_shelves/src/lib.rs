pub struct Solution {}

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let mut dp = vec![std::i32::MAX; books.len() + 1];
        *dp.last_mut().unwrap() = 0;

        for i in (0..books.len()).rev() {
            let mut w = 0;
            let mut h = 0;
            for j in (0..=i).rev() {
                w += books[j][0];
                h = h.max(books[j][1]);
                if shelf_width < w {
                    break;
                }
                dp[j] = dp[j].min(dp[i + 1] + h);
            }
        }

        dp[0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1105() {
        assert_eq!(
            Solution::min_height_shelves(
                vec![
                    vec![1, 1],
                    vec![2, 3],
                    vec![2, 3],
                    vec![1, 1],
                    vec![1, 1],
                    vec![1, 1],
                    vec![1, 2]
                ],
                4
            ),
            6
        );
        assert_eq!(
            Solution::min_height_shelves(vec![vec![1, 3], vec![2, 4], vec![3, 2]], 6),
            4
        );
    }
}
