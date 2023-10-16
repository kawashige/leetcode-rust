pub struct Solution {}

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut in_count = vec![0; n as usize];
        let mut outbound = vec![vec![]; n as usize];

        for edge in edges {
            in_count[edge[1] as usize] += 1;
            outbound[edge[0] as usize].push(edge[1] as usize);
        }

        let mut nodes = (0..in_count.len())
            .filter(|i| in_count[*i] == 0)
            .collect::<Vec<_>>();

        let mut result = vec![vec![false; n as usize]; n as usize];

        while let Some(node) = nodes.pop() {
            for next in &outbound[node] {
                in_count[*next] -= 1;
                if in_count[*next] == 0 {
                    nodes.push(*next);
                }
                result[*next][node] = true;
                for i in 0..n as usize {
                    result[*next][i] |= result[node][i];
                }
            }
        }

        result
            .into_iter()
            .map(|v| (0..n).filter(|i| v[*i as usize]).collect::<Vec<_>>())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2192() {
        assert_eq!(
            Solution::get_ancestors(
                8,
                vec![
                    vec![0, 3],
                    vec![0, 4],
                    vec![1, 3],
                    vec![2, 4],
                    vec![2, 7],
                    vec![3, 5],
                    vec![3, 6],
                    vec![3, 7],
                    vec![4, 6]
                ]
            ),
            vec![
                vec![],
                vec![],
                vec![],
                vec![0, 1],
                vec![0, 2],
                vec![0, 1, 3],
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 2, 3]
            ]
        );
        assert_eq!(
            Solution::get_ancestors(
                5,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![0, 4],
                    vec![1, 2],
                    vec![1, 3],
                    vec![1, 4],
                    vec![2, 3],
                    vec![2, 4],
                    vec![3, 4]
                ]
            ),
            vec![vec![], vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1, 2, 3]]
        );
    }
}
