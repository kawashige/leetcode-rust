pub struct Solution {}

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let sums = std::iter::once(0)
            .chain(card_points.into_iter().scan(0, |sum, x| {
                *sum += x;
                Some(*sum)
            }))
            .collect::<Vec<i32>>();

        let n = sums.len() - 1;
        let k = k as usize;
        (0..=k).fold(0, |max, i| max.max(sums[i] + sums[n] - sums[n + i - k]))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day11() {
        assert_eq!(Solution::max_score(vec![100, 40, 17, 9, 73, 75], 3), 248);
        assert_eq!(Solution::max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
        assert_eq!(Solution::max_score(vec![2, 2, 2], 2), 4);
        assert_eq!(Solution::max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
        assert_eq!(Solution::max_score(vec![1, 1000, 1], 1), 1);
        assert_eq!(
            Solution::max_score(vec![1, 79, 80, 1, 1, 1, 200, 1], 3),
            202
        );
    }
}
