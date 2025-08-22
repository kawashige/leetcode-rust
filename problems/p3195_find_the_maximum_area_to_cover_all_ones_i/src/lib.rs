pub struct Solution {}

impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut r = [std::usize::MAX, std::usize::MIN];
        let mut c = [std::usize::MAX, std::usize::MIN];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    r[0] = r[0].min(i);
                    r[1] = r[1].max(i);
                    c[0] = c[0].min(j);
                    c[1] = c[1].max(j);
                }
            }
        }

        ((r[1] - r[0] + 1) * (c[1] - c[0] + 1)) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3195() {
        assert_eq!(
            Solution::minimum_area(vec![vec![0, 1, 0], vec![1, 0, 1]]),
            6
        );
        assert_eq!(Solution::minimum_area(vec![vec![1, 0], vec![0, 0]]), 1);
    }
}
