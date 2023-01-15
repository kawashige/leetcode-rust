use std::collections::BTreeMap;

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
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let mut edge_vals = edges
            .into_iter()
            .map(|edge| (vals[edge[0] as usize].max(vals[edge[1] as usize]), edge))
            .collect::<Vec<_>>();
        edge_vals.sort_unstable();

        let mut vals_index = BTreeMap::new();
        for i in 0..vals.len() {
            vals_index.entry(vals[i]).or_insert(vec![]).push(i);
        }

        let mut disjoint_set = DisjointSet::new(vals.len());
        let mut count = vals.len();
        let mut i = 0;

        for (val, indices) in vals_index {
            let mut counts = vec![0; vals.len()];
            while i < edge_vals.len() && edge_vals[i].0 <= val {
                disjoint_set.unite(edge_vals[i].1[0] as usize, edge_vals[i].1[1] as usize);
                i += 1;
            }

            for j in indices {
                counts[disjoint_set.root(j)] += 1;
                count += counts[disjoint_set.root(j)] - 1;
            }
        }

        count as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2421() {
        assert_eq!(
            Solution::number_of_good_paths(
                vec![1, 3, 2, 1, 3],
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]]
            ),
            6
        );
        assert_eq!(
            Solution::number_of_good_paths(
                vec![1, 1, 2, 2, 3],
                vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4]]
            ),
            7
        );
        assert_eq!(Solution::number_of_good_paths(vec![1], vec![]), 1);
        assert_eq!(
            Solution::number_of_good_paths(
                vec![2, 5, 5, 1, 5, 2, 3, 5, 1, 5],
                vec![
                    vec![0, 1],
                    vec![2, 1],
                    vec![3, 2],
                    vec![3, 4],
                    vec![3, 5],
                    vec![5, 6],
                    vec![1, 7],
                    vec![8, 4],
                    vec![9, 7]
                ]
            ),
            20
        );
    }
}
