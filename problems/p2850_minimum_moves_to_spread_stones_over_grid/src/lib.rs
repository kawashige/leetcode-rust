pub struct Solution {}

impl Solution {
    pub fn recurse(
        i: usize,
        moves: i32,
        used: &mut Vec<bool>,
        zero: &Vec<(i32, i32)>,
        multiple: &Vec<(i32, i32)>,
        min_moves: &mut i32,
    ) {
        if i == zero.len() {
            if &moves < min_moves {
                *min_moves = moves;
            }
            return;
        }

        for j in 0..multiple.len() {
            if used[j] {
                continue;
            }
            used[j] = true;
            Self::recurse(
                i + 1,
                moves + (zero[i].0 - multiple[j].0).abs() + (zero[i].1 - multiple[j].1).abs(),
                used,
                zero,
                multiple,
                min_moves,
            );
            used[j] = false;
        }
    }

    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut zero = Vec::new();
        let mut multiple = Vec::new();

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    zero.push((i as i32, j as i32));
                } else if 1 < grid[i][j] {
                    for _ in 0..grid[i][j] - 1 {
                        multiple.push((i as i32, j as i32));
                    }
                }
            }
        }

        if zero.is_empty() {
            return 0;
        }

        let mut result = std::i32::MAX;
        Self::recurse(
            0,
            0,
            &mut vec![false; multiple.len()],
            &zero,
            &multiple,
            &mut result,
        );
        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2850() {
        assert_eq!(
            Solution::minimum_moves(vec![vec![1, 1, 0], vec![1, 1, 1], vec![1, 2, 1]]),
            3
        );
        assert_eq!(
            Solution::minimum_moves(vec![vec![1, 3, 0], vec![1, 0, 0], vec![1, 0, 3]]),
            4
        );
    }
}
