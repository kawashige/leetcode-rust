pub struct Solution {}

impl Solution {
    pub fn recurse(
        node_count: usize,
        edges: &Vec<Vec<i32>>,
        node_added: &mut Vec<bool>,
        used_edge: &mut Vec<bool>,
        critical_edge: &mut Vec<bool>,
        preude_critical_edge: &mut Vec<bool>,
        cost: i32,
        min_cost: i32,
    ) {
        if node_count == node_added.len() {
            for i in 0..used_edge.len() {
                if used_edge[i] {
                    preude_critical_edge[i] = true;
                } else {
                    critical_edge[i] = false
                }
            }
            return;
        }

        for i in 0..edges.len() {
            if used_edge[i] {
                continue;
            }
            if node_added[edges[i][0] as usize]
                && !node_added[edges[i][1] as usize]
                && cost + edges[i][2] <= min_cost
            {
                node_added[edges[i][1] as usize] = true;
                used_edge[i] = true;
                Self::recurse(
                    node_count + 1,
                    edges,
                    node_added,
                    used_edge,
                    critical_edge,
                    preude_critical_edge,
                    cost + edges[i][2],
                    min_cost,
                );
                node_added[edges[i][1] as usize] = false;
                used_edge[i] = false;
            } else if !node_added[edges[i][0] as usize]
                && node_added[edges[i][1] as usize]
                && cost + edges[i][2] <= min_cost
            {
                node_added[edges[i][0] as usize] = true;
                used_edge[i] = true;
                Self::recurse(
                    node_count + 1,
                    edges,
                    node_added,
                    used_edge,
                    critical_edge,
                    preude_critical_edge,
                    cost + edges[i][2],
                    min_cost,
                );
                node_added[edges[i][0] as usize] = false;
                used_edge[i] = false;
            }
        }
    }

    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut sorted_edges = edges.iter().cloned().zip(0..).collect::<Vec<_>>();
        sorted_edges.sort_unstable_by(|a, b| a.0[2].cmp(&b.0[2]));

        let mut added = vec![false; n as usize];
        added[0] = true;
        let mut min_cost = 0;
        let mut critical_edge = vec![false; sorted_edges.len()];

        for _ in 0..n - 1 {
            for i in 0..sorted_edges.len() {
                if added[sorted_edges[i].0[0] as usize] && !added[sorted_edges[i].0[1] as usize] {
                    added[sorted_edges[i].0[1] as usize] = true;
                    min_cost += sorted_edges[i].0[2];
                    critical_edge[sorted_edges[i].1] = true;
                    break;
                } else if !added[sorted_edges[i].0[0] as usize]
                    && added[sorted_edges[i].0[1] as usize]
                {
                    added[sorted_edges[i].0[0] as usize] = true;
                    min_cost += sorted_edges[i].0[2];
                    critical_edge[sorted_edges[i].1] = true;
                    break;
                }
            }
        }

        let mut preude_critical_edge = vec![false; edges.len()];
        let mut used_edge = vec![false; edges.len()];
        let mut node_added = vec![false; n as usize];
        node_added[0] = true;

        Self::recurse(
            1,
            &edges,
            &mut node_added,
            &mut used_edge,
            &mut critical_edge,
            &mut preude_critical_edge,
            0,
            min_cost,
        );

        let mut result = vec![vec![], vec![]];

        for i in 0..edges.len() {
            if critical_edge[i] {
                result[0].push(i as i32);
            } else if preude_critical_edge[i] {
                result[1].push(i as i32);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1489() {
        assert_eq!(
            Solution::find_critical_and_pseudo_critical_edges(
                5,
                vec![
                    vec![0, 1, 1],
                    vec![1, 2, 1],
                    vec![2, 3, 2],
                    vec![0, 3, 2],
                    vec![0, 4, 3],
                    vec![3, 4, 3],
                    vec![1, 4, 6]
                ]
            ),
            vec![vec![0, 1], vec![2, 3, 4, 5]]
        );
        assert_eq!(
            Solution::find_critical_and_pseudo_critical_edges(
                4,
                vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 1], vec![0, 3, 1]]
            ),
            vec![vec![], vec![0, 1, 2, 3]]
        );
    }
}
