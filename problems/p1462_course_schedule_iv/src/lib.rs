pub struct Solution {}

impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut all_prerequisites = vec![vec![false; num_courses as usize]; num_courses as usize];
        let mut in_count = vec![0; num_courses as usize];
        let mut matrix = vec![vec![false; num_courses as usize]; num_courses as usize];

        for pre in prerequisites {
            all_prerequisites[pre[1] as usize][pre[0] as usize] = true;
            matrix[pre[0] as usize][pre[1] as usize] = true;
            in_count[pre[1] as usize] += 1;
        }

        let mut indices = (0..in_count.len())
            .filter(|i| in_count[*i] == 0)
            .collect::<Vec<_>>();

        while let Some(i) = indices.pop() {
            for j in (0..matrix.len()).filter(|j| matrix[*j][i]) {
                for k in 0..all_prerequisites.len() {
                    all_prerequisites[i][k] |= all_prerequisites[j][k];
                }
            }

            for j in (0..matrix.len()).filter(|j| matrix[i][*j]) {
                in_count[j] -= 1;
                if in_count[j] == 0 {
                    indices.push(j);
                }
            }
        }

        queries
            .into_iter()
            .map(|q| all_prerequisites[q[1] as usize][q[0] as usize])
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1462() {
        assert_eq!(
            Solution::check_if_prerequisite(2, vec![vec![1, 0]], vec![vec![0, 1], vec![1, 0]]),
            vec![false, true]
        );
        assert_eq!(
            Solution::check_if_prerequisite(2, vec![], vec![vec![1, 0], vec![0, 1]]),
            vec![false, false]
        );
        assert_eq!(
            Solution::check_if_prerequisite(
                3,
                vec![vec![1, 2], vec![1, 0], vec![2, 0]],
                vec![vec![1, 0], vec![1, 2]]
            ),
            vec![true, true]
        );
    }
}
