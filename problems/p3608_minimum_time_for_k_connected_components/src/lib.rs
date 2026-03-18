pub struct Solution {}
pub struct DisjointSet {
    parent: Vec<usize>,
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
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        if edges.is_empty() {
            return 0;
        }
        let mut edges = edges;
        edges.sort_unstable_by_key(|e| e[2]);

        let mut result = edges.last().unwrap()[2];
        let mut components = n;
        let mut ds = DisjointSet::new(n as usize);

        while let Some(edge) = edges.pop() {
            if ds.root(edge[0] as usize) != ds.root(edge[1] as usize) {
                components -= 1;
                if components < k {
                    break;
                }
                ds.unite(edge[0] as usize, edge[1] as usize);
            }
            result = if edges.is_empty() {
                0
            } else {
                edges.last().unwrap()[2]
            };
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3608() {
        assert_eq!(Solution::min_time(2, vec![vec![0, 1, 3]], 2), 3);
        assert_eq!(
            Solution::min_time(3, vec![vec![0, 1, 2], vec![1, 2, 4]], 3),
            4
        );
        assert_eq!(Solution::min_time(3, vec![vec![0, 2, 5]], 2), 0);
    }
}
