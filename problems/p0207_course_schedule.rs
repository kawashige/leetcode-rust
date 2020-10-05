pub struct Solution {}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        fn is_cycle(matrix: &mut Vec<Vec<usize>>, routes: Vec<usize>, i: usize) -> bool {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 1 {
                    if routes.contains(&j) {
                        return true;
                    } else {
                        matrix[i][j] = 0;
                        let mut new_routes = routes.clone();
                        new_routes.push(j);
                        if is_cycle(matrix, new_routes, j) {
                            return true;
                        }
                    }
                }
            }
            false
        }

        let mut edges = vec![vec![0 as usize; num_courses as usize]; num_courses as usize];
        for p in prerequisites {
            edges[p[0] as usize][p[1] as usize] = 1;
        }

        (0..edges.len()).all(|i| !is_cycle(&mut edges, vec![i], i))
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0207() {
        assert!(Solution::can_finish(2, vec![vec![1, 0]]));
        assert!(!Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
        assert!(!Solution::can_finish(
            4,
            vec![vec![0, 1], vec![3, 1], vec![1, 3], vec![3, 2]]
        ));
        assert!(!Solution::can_finish(
            4,
            vec![vec![2, 0], vec![1, 0], vec![3, 1], vec![3, 2], vec![1, 3]]
        ));
    }
}
