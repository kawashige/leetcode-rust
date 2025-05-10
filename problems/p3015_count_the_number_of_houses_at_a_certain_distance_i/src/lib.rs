pub struct Solution {}

impl Solution {
    pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i32> {
        let mut dist = vec![vec![n as usize; n as usize]; n as usize];
        for i in 0..n as usize {
            dist[i][i] = 0;
            if i < n as usize - 1 {
                dist[i][i + 1] = 1;
                dist[i + 1][i] = 1;
            }
        }
        dist[x as usize - 1][y as usize - 1] = 1;
        dist[y as usize - 1][x as usize - 1] = 1;

        for k in 0..n as usize {
            for i in 0..n as usize {
                for j in 0..n as usize {
                    if dist[i][k] + dist[k][j] < dist[i][j] {
                        dist[i][j] = dist[i][k] + dist[k][j];
                    }
                }
            }
        }

        let mut result = vec![0; n as usize];
        for i in 0..n as usize {
            for j in 0..n as usize {
                if i != j && dist[i][j] < result.len() {
                    result[dist[i][j] as usize - 1] += 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3015() {
        assert_eq!(Solution::count_of_pairs(3, 1, 3), vec![6, 0, 0]);
        assert_eq!(Solution::count_of_pairs(5, 2, 4), vec![10, 8, 2, 0, 0]);
        assert_eq!(Solution::count_of_pairs(4, 1, 1), vec![6, 4, 2, 0]);
    }
}
