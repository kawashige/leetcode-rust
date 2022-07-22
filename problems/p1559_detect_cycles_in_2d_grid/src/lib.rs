pub struct Solution {}

impl Solution {
    pub fn recurse(
        i: usize,
        j: usize,
        prev_i: usize,
        prev_j: usize,
        grid: &Vec<Vec<char>>,
        seen: &mut Vec<Vec<bool>>,
    ) -> bool {
        seen[i][j] = true;
        for (d_i, d_j) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (new_i, new_j) = (i as i32 + d_i, j as i32 + d_j);
            if new_i < 0
                || grid.len() as i32 <= new_i
                || new_j < 0
                || grid[0].len() as i32 <= new_j
                || (new_i as usize == prev_i && new_j as usize == prev_j)
                || grid[new_i as usize][new_j as usize] != grid[i][j]
            {
                continue;
            }
            if seen[new_i as usize][new_j as usize] {
                return true;
            }
            if Self::recurse(new_i as usize, new_j as usize, i, j, grid, seen) {
                return true;
            }
        }
        false
    }

    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if !seen[i][j] {
                    if Self::recurse(i, j, grid.len(), grid[0].len(), &grid, &mut seen) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1559() {
        assert!(!Solution::contains_cycle(vec![
            vec!['a', 'b', 'b'],
            vec!['b', 'z', 'b'],
            vec!['b', 'b', 'a']
        ]));
        assert!(Solution::contains_cycle(vec![
            vec!['a', 'a', 'a', 'a'],
            vec!['a', 'b', 'b', 'a'],
            vec!['a', 'b', 'b', 'a'],
            vec!['a', 'a', 'a', 'a']
        ]));
        assert!(Solution::contains_cycle(vec![
            vec!['c', 'c', 'c', 'a'],
            vec!['c', 'd', 'c', 'c'],
            vec!['c', 'c', 'e', 'c'],
            vec!['f', 'c', 'c', 'c']
        ]));
        assert!(!Solution::contains_cycle(vec![
            vec!['a', 'b', 'b'],
            vec!['b', 'z', 'b'],
            vec!['b', 'b', 'a']
        ]));
    }
}
