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
    pub fn distance_limited_paths_exist(
        n: i32,
        edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut edge_list = edge_list;
        let mut queries = queries.into_iter().enumerate().collect::<Vec<_>>();
        edge_list.sort_unstable_by(|a, b| a[2].cmp(&b[2]));
        queries.sort_unstable_by(|(_, a), (_, b)| a[2].cmp(&b[2]));

        let mut result = vec![false; queries.len()];
        let mut disjoint_set = DisjointSet::new(n as usize);
        let mut i = 0;

        for (j, query) in queries {
            while i < edge_list.len() && edge_list[i][2] < query[2] {
                disjoint_set.unite(edge_list[i][0] as usize, edge_list[i][1] as usize);
                i += 1;
            }
            result[j] =
                disjoint_set.root(query[0] as usize) == disjoint_set.root(query[1] as usize);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1697() {
        assert_eq!(
            Solution::distance_limited_paths_exist(
                3,
                vec![vec![0, 1, 2], vec![1, 2, 4], vec![2, 0, 8], vec![1, 0, 16]],
                vec![vec![0, 1, 2], vec![0, 2, 5]]
            ),
            vec![false, true]
        );
        assert_eq!(
            Solution::distance_limited_paths_exist(
                5,
                vec![vec![0, 1, 10], vec![1, 2, 5], vec![2, 3, 9], vec![3, 4, 13]],
                vec![vec![0, 4, 14], vec![1, 4, 13]]
            ),
            vec![true, false]
        );
    }
}
