pub struct Solution {}

impl Solution {
    pub fn count_cells(grid: Vec<Vec<char>>, pattern: String) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        const MOD: i64 = i32::MAX as i64;
        // rabin karp to detect pattern
        let mut target = 0i64;
        for &c in pattern.as_bytes() {
            let c = c as i64 - 97;
            target = ((target * 26) % MOD + c) % MOD;
        }
        let mut pow = 1;
        for _ in 0..pattern.len() - 1 {
            pow = (pow * 26) % MOD;
        }
        let mut grid_pattern = vec![0; n * m];

        // run horizontal
        let mut last = 0;
        let mut cur = 0;
        for i in 0..m * n {
            let (r, c) = (i / n, i % n);
            let c = grid[r][c] as i64 - 97;
            if i < pattern.len() {
                cur = ((cur * 26) % MOD + c) % MOD;
            } else {
                let pi = i - pattern.len();
                let (pr, pc) = (pi / n, pi % n);
                let pc = grid[pr][pc] as i64 - 97;

                cur = ((cur - (pow * pc) % MOD + MOD) * 26) % MOD + c;
            }
            if i >= pattern.len() - 1 && cur == target {
                let start = (i as i32 - pattern.len() as i32 + 1).max(last as i32) as usize;
                for j in start..=i {
                    grid_pattern[j] = 1;
                }
                last = i;
            }
        }

        // run vertical
        let mut last = 0;
        let mut cur = 0;
        for i in 0..m * n {
            let (r, c) = (i % m, i / m);
            let c = grid[r][c] as i64 - 97;
            if i < pattern.len() {
                cur = ((cur * 26) % MOD + c) % MOD;
            } else {
                let pi = i - pattern.len();
                let (pr, pc) = (pi % m, pi / m);
                let pc = grid[pr][pc] as i64 - 97;

                cur = ((cur - (pow * pc) % MOD + MOD) * 26) % MOD + c;
            }
            if i >= pattern.len() - 1 && cur == target {
                let start = (i as i32 - pattern.len() as i32 + 1).max(last as i32) as usize;
                for j in start..=i {
                    let (r, c) = (j % m, j / m);
                    let j = r * n + c;
                    grid_pattern[j] |= 2;
                }
                last = i;
            }
        }

        // check if row and col is matching
        grid_pattern
            .iter()
            .map(|&x| if x == 3 { 1 } else { 0 })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3529() {
        assert_eq!(
            Solution::count_cells(
                vec![
                    vec!['a', 'a', 'c', 'c'],
                    vec!['b', 'b', 'b', 'c'],
                    vec!['a', 'a', 'b', 'a'],
                    vec!['c', 'a', 'a', 'c'],
                    vec!['a', 'a', 'b', 'a']
                ],
                "abaca".to_string()
            ),
            1
        );
        assert_eq!(
            Solution::count_cells(
                vec![
                    vec!['c', 'a', 'a', 'a'],
                    vec!['a', 'a', 'b', 'a'],
                    vec!['b', 'b', 'a', 'a'],
                    vec!['a', 'a', 'b', 'a']
                ],
                "aba".to_string()
            ),
            4
        );
        assert_eq!(Solution::count_cells(vec![vec!['a']], "a".to_string()), 1);
    }
}
