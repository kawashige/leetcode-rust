pub struct Solution {}

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let d = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        let mut i = 0;
        let mut l = 1;
        let mut p = vec![r_start, c_start];

        let n = (rows * cols) as usize;
        let mut r = Vec::with_capacity(n);
        r.push(p.clone());

        while r.len() < n {
            for _ in 0..l {
                p[0] += d[i][0];
                p[1] += d[i][1];
                if 0 <= p[0] && p[0] < rows && 0 <= p[1] && p[1] < cols {
                    r.push(p.clone());
                    if r.len() == n {
                        break;
                    }
                }
            }
            if i % 2 == 1 {
                l += 1;
            }
            i = (i + 1) % 4;
        }

        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0885() {
        assert_eq!(
            Solution::spiral_matrix_iii(1, 4, 0, 0),
            vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]]
        );
        assert_eq!(
            Solution::spiral_matrix_iii(5, 6, 1, 4),
            vec![
                vec![1, 4],
                vec![1, 5],
                vec![2, 5],
                vec![2, 4],
                vec![2, 3],
                vec![1, 3],
                vec![0, 3],
                vec![0, 4],
                vec![0, 5],
                vec![3, 5],
                vec![3, 4],
                vec![3, 3],
                vec![3, 2],
                vec![2, 2],
                vec![1, 2],
                vec![0, 2],
                vec![4, 5],
                vec![4, 4],
                vec![4, 3],
                vec![4, 2],
                vec![4, 1],
                vec![3, 1],
                vec![2, 1],
                vec![1, 1],
                vec![0, 1],
                vec![4, 0],
                vec![3, 0],
                vec![2, 0],
                vec![1, 0],
                vec![0, 0]
            ]
        );
    }
}
