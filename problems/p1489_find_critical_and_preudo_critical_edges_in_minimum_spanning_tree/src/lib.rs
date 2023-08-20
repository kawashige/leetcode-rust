pub struct Solution {}

pub struct DisjointSet {
    pub parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn root(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }

        let parent = self.root(self.parent[i]);
        self.parent[i] = parent;
        parent
    }

    pub fn unite(&mut self, i: usize, j: usize) {
        let mut parent_i = self.root(i);
        let mut parent_j = self.root(j);

        if parent_i == parent_j {
            return;
        }

        if self.size(parent_i) < self.size(parent_j) {
            std::mem::swap(&mut parent_i, &mut parent_j);
        }

        self.parent[parent_j] = parent_i;
        self.size[parent_i] += self.size[parent_j];
    }

    pub fn size(&mut self, i: usize) -> usize {
        let root = self.root(i);
        self.size[root]
    }
}
impl Solution {
    pub fn find_mst(n: usize, edges: &Vec<(Vec<i32>, usize)>, initial_edges: Option<usize>) -> i32 {
        let mut disjoint_set = DisjointSet::new(n as usize);
        let mut cost = 0;
        let mut root_node = 0;

        if let Some(i) = initial_edges {
            cost += edges[i].0[2];
            root_node = edges[i].0[0] as usize;
            disjoint_set.unite(root_node, edges[i].0[1] as usize);
        }

        while disjoint_set.size(root_node) < n {
            let mut found = false;
            for i in 0..edges.len() {
                if disjoint_set.root(root_node) == disjoint_set.root(edges[i].0[0] as usize)
                    && disjoint_set.root(root_node) != disjoint_set.root(edges[i].0[1] as usize)
                {
                    disjoint_set.unite(root_node, edges[i].0[1] as usize);
                    cost += edges[i].0[2];
                    found = true;
                    break;
                } else if disjoint_set.root(root_node) != disjoint_set.root(edges[i].0[0] as usize)
                    && disjoint_set.root(root_node) == disjoint_set.root(edges[i].0[1] as usize)
                {
                    disjoint_set.unite(root_node, edges[i].0[0] as usize);
                    cost += edges[i].0[2];
                    found = true;
                    break;
                }
            }
            if !found {
                break;
            }
        }

        if disjoint_set.size(root_node) != n {
            std::i32::MAX
        } else {
            cost
        }
    }

    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut sorted_edges = edges.iter().cloned().zip(0..).collect::<Vec<_>>();
        sorted_edges.sort_unstable_by(|a, b| a.0[2].cmp(&b.0[2]));

        let min_cost = Self::find_mst(n as usize, &sorted_edges, None);
        let mut critical_edges = Vec::new();
        let mut pseude_critical_edges = Vec::new();

        for i in 0..sorted_edges.len() {
            let mut deleted_edges = sorted_edges.clone();
            deleted_edges.remove(i);

            if min_cost < Self::find_mst(n as usize, &deleted_edges, None) {
                critical_edges.push(sorted_edges[i].1 as i32);
            } else {
                if min_cost == Self::find_mst(n as usize, &sorted_edges, Some(i)) {
                    pseude_critical_edges.push(sorted_edges[i].1 as i32);
                }
            }
        }

        vec![critical_edges, pseude_critical_edges]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1489() {
        assert_eq!(
            Solution::find_critical_and_pseudo_critical_edges(
                14,
                vec![
                    vec![0, 1, 13],
                    vec![0, 2, 6],
                    vec![2, 3, 13],
                    vec![3, 4, 4],
                    vec![0, 5, 11],
                    vec![4, 6, 14],
                    vec![4, 7, 8],
                    vec![2, 8, 6],
                    vec![4, 9, 6],
                    vec![7, 10, 4],
                    vec![5, 11, 3],
                    vec![6, 12, 7],
                    vec![12, 13, 9],
                    vec![7, 13, 2],
                    vec![5, 13, 10],
                    vec![0, 6, 4],
                    vec![2, 7, 3],
                    vec![0, 7, 8],
                    vec![1, 12, 9],
                    vec![10, 12, 11],
                    vec![1, 2, 7],
                    vec![1, 3, 10],
                    vec![3, 10, 6],
                    vec![6, 10, 4],
                    vec![4, 8, 5],
                    vec![1, 13, 4],
                    vec![11, 13, 8],
                    vec![2, 12, 10],
                    vec![5, 8, 1],
                    vec![3, 7, 6],
                    vec![7, 12, 12],
                    vec![1, 7, 9],
                    vec![5, 9, 1],
                    vec![2, 13, 10],
                    vec![10, 11, 4],
                    vec![3, 5, 10],
                    vec![6, 11, 14],
                    vec![5, 12, 3],
                    vec![0, 8, 13],
                    vec![8, 9, 1],
                    vec![3, 6, 8],
                    vec![0, 3, 4],
                    vec![2, 9, 6],
                    vec![0, 11, 4],
                    vec![2, 5, 14],
                    vec![4, 11, 2],
                    vec![7, 11, 11],
                    vec![1, 11, 6],
                    vec![2, 10, 12],
                    vec![0, 13, 4],
                    vec![3, 9, 9],
                    vec![4, 12, 3],
                    vec![6, 7, 10],
                    vec![6, 8, 13],
                    vec![9, 11, 3],
                    vec![1, 6, 2],
                    vec![2, 4, 12],
                    vec![0, 10, 3],
                    vec![3, 12, 1],
                    vec![3, 8, 12],
                    vec![1, 8, 6],
                    vec![8, 13, 2],
                    vec![10, 13, 12],
                    vec![9, 13, 11],
                    vec![2, 11, 14],
                    vec![5, 10, 9],
                    vec![5, 6, 10],
                    vec![2, 6, 9],
                    vec![4, 10, 7],
                    vec![3, 13, 10],
                    vec![4, 13, 3],
                    vec![3, 11, 9],
                    vec![7, 9, 14],
                    vec![6, 9, 5],
                    vec![1, 5, 12],
                    vec![4, 5, 3],
                    vec![11, 12, 3],
                    vec![0, 4, 8],
                    vec![5, 7, 8],
                    vec![9, 12, 13],
                    vec![8, 12, 12],
                    vec![1, 10, 6],
                    vec![1, 9, 9],
                    vec![7, 8, 9],
                    vec![9, 10, 13],
                    vec![8, 11, 3],
                    vec![6, 13, 7],
                    vec![0, 12, 10],
                    vec![1, 4, 8],
                    vec![8, 10, 2]
                ]
            ),
            vec![vec![0, 2, 3, 5], vec![1, 4]]
        );
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
