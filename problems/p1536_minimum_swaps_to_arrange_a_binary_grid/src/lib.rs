pub struct Solution {}

impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let l = grid[0].len() - 1;
        let mut right_zeros = grid
            .into_iter()
            .map(|row| row.into_iter().rev().take_while(|x| x == &0).count())
            .collect::<Vec<_>>();

        let mut swaps = 0;
        for i in 0..right_zeros.len() {
            if right_zeros[i] < l - i {
                if let Some(j) = (i + 1..right_zeros.len()).find(|j| l - i <= right_zeros[*j]) {
                    swaps += j - i;
                    let target = right_zeros.remove(j);
                    right_zeros.insert(i, target);
                } else {
                    return -1;
                }
            }
        }
        swaps as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1536() {
        assert_eq!(
            Solution::min_swaps(vec![vec![0, 0, 1], vec![1, 1, 0], vec![1, 0, 0]]),
            3
        );
        assert_eq!(
            Solution::min_swaps(vec![
                vec![0, 1, 1, 0],
                vec![0, 1, 1, 0],
                vec![0, 1, 1, 0],
                vec![0, 1, 1, 0]
            ]),
            -1
        );
        assert_eq!(
            Solution::min_swaps(vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 1]]),
            0
        );
    }
}
