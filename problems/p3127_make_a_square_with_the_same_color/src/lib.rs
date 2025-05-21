pub struct Solution {}

impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        for i in 0..2 {
            for j in 0..2 {
                let mut count = [0; 2];
                for (di, dj) in [(0, 0), (0, 1), (1, 0), (1, 1)].iter() {
                    let (x, y) = (i + di, j + dj);
                    count[if grid[x][y] == 'B' { 0 } else { 1 }] += 1;
                }
                if count[0] != 2 {
                    return true;
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
    fn test_3127() {
        assert!(Solution::can_make_square(vec![
            vec!['B', 'W', 'B'],
            vec!['B', 'W', 'W'],
            vec!['B', 'W', 'B']
        ]));
        assert!(!Solution::can_make_square(vec![
            vec!['B', 'W', 'B'],
            vec!['W', 'B', 'W'],
            vec!['B', 'W', 'B']
        ]));
        assert!(Solution::can_make_square(vec![
            vec!['B', 'W', 'B'],
            vec!['B', 'W', 'W'],
            vec!['B', 'W', 'W']
        ]));
    }
}
