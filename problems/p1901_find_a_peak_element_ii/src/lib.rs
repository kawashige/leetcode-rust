pub struct Solution {}

impl Solution {
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut r, mut c) = (0, 0);

        while let Some((new_r, new_c)) = [(-1, 0), (0, -1), (0, 1), (1, 0)]
            .iter()
            .filter_map(|(dr, dc)| {
                let (new_r, new_c) = (r as i32 + dr, c as i32 + dc);
                if new_r < 0
                    || new_c < 0
                    || mat.len() as i32 <= new_r
                    || mat[0].len() as i32 <= new_c
                    || mat[new_r as usize][new_c as usize] < mat[r][c]
                {
                    None
                } else {
                    Some((new_r as usize, new_c as usize))
                }
            })
            .max_by_key(|(new_r, new_c)| mat[*new_r][*new_c])
        {
            r = new_r;
            c = new_c;
        }

        vec![r as i32, c as i32]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1901() {
        assert_eq!(
            Solution::find_peak_grid(vec![vec![1, 4], vec![3, 2]]),
            vec![0, 1]
        );
        assert_eq!(
            Solution::find_peak_grid(vec![vec![10, 20, 15], vec![21, 30, 14], vec![7, 16, 32]]),
            vec![1, 1]
        );
    }
}
