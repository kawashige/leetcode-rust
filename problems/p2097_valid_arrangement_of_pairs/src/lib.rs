use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn dfs(start: i32, next_nodes: &mut HashMap<i32, Vec<i32>>) -> Vec<i32> {
        let mut routes = Vec::new();
        let mut nodes = vec![start];
        while let Some(node) = nodes.pop() {
            routes.push(node);

            if !next_nodes.contains_key(&node) {
                continue;
            }
            if let Some(next_node) = next_nodes.get_mut(&node).unwrap().pop() {
                nodes.push(next_node);
            }
        }
        return routes;
    }

    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut count = HashMap::new();
        let mut next_nodes = HashMap::new();

        for p in &pairs {
            *count.entry(p[0]).or_insert(0) -= 1;
            *count.entry(p[1]).or_insert(0) += 1;
            (*next_nodes.entry(p[0]).or_insert(Vec::new())).push(p[1]);
        }

        let start = *count.keys().find(|k| count[*k] < 0).unwrap_or(&pairs[0][0]);

        let mut result = Vec::new();
        let mut routes = Self::dfs(start, &mut next_nodes)
            .into_iter()
            .rev()
            .collect::<Vec<_>>();

        while let Some(node) = routes.pop() {
            if !next_nodes.contains_key(&node) || next_nodes[&node].is_empty() {
                result.push(node);
                continue;
            }
            while next_nodes.contains_key(&node) && !next_nodes[&node].is_empty() {
                let new_routes = &mut Self::dfs(node, &mut next_nodes);
                for i in (0..new_routes.len()).rev() {
                    routes.push(new_routes[i]);
                }
            }
        }

        (0..result.len() - 1)
            .map(|i| vec![result[i], result[i + 1]])
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2097() {
        assert_eq!(
            Solution::valid_arrangement(vec![vec![5, 1], vec![4, 5], vec![11, 9], vec![9, 4]]),
            vec![vec![11, 9], vec![9, 4], vec![4, 5], vec![5, 1]]
        );
        assert_eq!(
            Solution::valid_arrangement(vec![vec![1, 3], vec![3, 2], vec![2, 1]]),
            vec![vec![1, 3], vec![3, 2], vec![2, 1]]
        );
        assert_eq!(
            Solution::valid_arrangement(vec![vec![1, 2], vec![1, 3], vec![2, 1]]),
            vec![vec![1, 2], vec![2, 1], vec![1, 3]]
        );
    }
}
