use std::collections::HashSet;

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
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut edges = edges;
        edges.sort_unstable_by(|a, b| b[0].cmp(&a[0]));

        let mut alice_set = DisjointSet::new(n as usize);
        let mut bob_set = DisjointSet::new(n as usize);
        let mut count = 0;

        for edge in edges {
            let mut united = false;
            if edge[0] != 2
                && (alice_set.root(edge[1] as usize - 1) != alice_set.root(edge[2] as usize - 1))
            {
                alice_set.unite(edge[1] as usize - 1, edge[2] as usize - 1);
                united = true;
            }
            if edge[0] != 1
                && (bob_set.root(edge[1] as usize - 1) != bob_set.root(edge[2] as usize - 1))
            {
                bob_set.unite(edge[1] as usize - 1, edge[2] as usize - 1);
                united = true;
            }
            if !united {
                count += 1;
            }
        }

        if (0..n as usize)
            .map(|i| alice_set.root(i))
            .collect::<HashSet<_>>()
            .len()
            == 1
            && (0..n as usize)
                .map(|i| bob_set.root(i))
                .collect::<HashSet<_>>()
                .len()
                == 1
        {
            count as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1579() {
        assert_eq!(
            Solution::max_num_edges_to_remove(2, vec![vec![1, 1, 2], vec![2, 1, 2], vec![3, 1, 2]]),
            2
        );
        assert_eq!(
            Solution::max_num_edges_to_remove(
                4,
                vec![
                    vec![3, 1, 2],
                    vec![3, 2, 3],
                    vec![1, 1, 3],
                    vec![1, 2, 4],
                    vec![1, 1, 2],
                    vec![2, 3, 4]
                ]
            ),
            2
        );
        assert_eq!(
            Solution::max_num_edges_to_remove(
                4,
                vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 4], vec![2, 1, 4]]
            ),
            0
        );
        assert_eq!(
            Solution::max_num_edges_to_remove(4, vec![vec![3, 2, 3], vec![1, 1, 2], vec![2, 3, 4]]),
            -1
        );
    }
}
