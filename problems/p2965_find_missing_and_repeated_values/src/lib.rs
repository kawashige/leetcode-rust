pub struct Solution {}

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut count = vec![0; grid.len() * grid.len() + 1];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                count[grid[i][j] as usize] += 1;
            }
        }
        let mut result = vec![0; 2];
        for i in 0..count.len() {
            if count[i] == 2 {
                result[0] = i as i32
            } else if count[i] == 0 {
                result[1] = i as i32
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2965() {
        assert_eq!(
            Solution::find_missing_and_repeated_values(vec![vec![1, 3], vec![2, 2]]),
            vec![2, 4]
        );
        assert_eq!(
            Solution::find_missing_and_repeated_values(vec![
                vec![9, 1, 7],
                vec![8, 9, 2],
                vec![3, 4, 6]
            ]),
            vec![9, 5]
        );
    }
}
