pub struct Solution {}

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums
            .into_iter()
            .enumerate()
            .map(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .map(move |(j, num)| (i + j, j, num))
            })
            .flatten()
            .collect::<Vec<_>>();

        nums.sort_unstable();
        nums.into_iter().map(|(_, _, num)| num).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1424() {
        assert_eq!(
            Solution::find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 4, 2, 7, 5, 3, 8, 6, 9]
        );
        assert_eq!(
            Solution::find_diagonal_order(vec![
                vec![1, 2, 3, 4, 5],
                vec![6, 7],
                vec![8],
                vec![9, 10, 11],
                vec![12, 13, 14, 15, 16]
            ]),
            vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16]
        );
    }
}
