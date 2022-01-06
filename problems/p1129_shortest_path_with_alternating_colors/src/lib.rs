pub struct Solution {}

impl Solution {
    pub fn dfs(
        i: usize,
        prev_color: usize,
        d: i32,
        dist: &mut Vec<Vec<i32>>,
        matrix: &Vec<Vec<Vec<bool>>>,
    ) {
        let next_color = (prev_color + 1) % 2;
        for j in 0..matrix[next_color].len() {
            if matrix[next_color][i][j] && d + 1 < dist[j][next_color] {
                dist[j][next_color] = d + 1;
                Self::dfs(j, next_color, d + 1, dist, matrix);
            }
        }
    }

    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let mut matrix = vec![
            vec![vec![false; n as usize]; n as usize],
            vec![vec![false; n as usize]; n as usize],
        ];
        for red_edge in red_edges {
            matrix[0][red_edge[0] as usize][red_edge[1] as usize] = true;
        }
        for blue_edge in blue_edges {
            matrix[1][blue_edge[0] as usize][blue_edge[1] as usize] = true;
        }

        let mut dist = vec![vec![std::i32::MAX; 2]; n as usize];
        dist[0][0] = 0;
        dist[0][1] = 0;

        Self::dfs(0, 0, 0, &mut dist, &matrix);
        Self::dfs(0, 1, 0, &mut dist, &matrix);

        dist.into_iter()
            .map(|d| {
                let min = d[0].min(d[1]);
                if min == std::i32::MAX {
                    -1
                } else {
                    min
                }
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1129() {
        assert_eq!(
            Solution::shortest_alternating_paths(3, vec![vec![0, 1]], vec![vec![1, 2]]),
            vec![0, 1, 2]
        );
        assert_eq!(
            Solution::shortest_alternating_paths(3, vec![vec![1, 0], vec![2, 1]], vec![]),
            vec![0, -1, -1]
        );
        assert_eq!(
            Solution::shortest_alternating_paths(3, vec![vec![0, 1], vec![1, 2]], vec![]),
            vec![0, 1, -1]
        );
        assert_eq!(
            Solution::shortest_alternating_paths(3, vec![vec![0, 1]], vec![vec![2, 1]]),
            vec![0, 1, -1]
        );
    }
}
