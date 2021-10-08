use std::collections::HashSet;

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
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let n = grid.len();
        let offset = n * n;
        let grid = grid
            .into_iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut ds = DisjointSet::new(n * n * 4);

        for i in 0..n {
            for j in 0..n {
                let idx = i * n + j;
                if i > 0 {
                    ds.unite(idx, ((i - 1) * n + j) + 2 * offset);
                }
                if j > 0 {
                    ds.unite(idx + 3 * offset, (i * n + j - 1) + offset);
                }
                match grid[i][j] {
                    '\\' => {
                        ds.unite(idx, idx + offset);
                        ds.unite(idx + 2 * offset, idx + 3 * offset);
                    }
                    '/' => {
                        ds.unite(idx, idx + 3 * offset);
                        ds.unite(idx + offset, idx + 2 * offset);
                    }
                    _ => {
                        ds.unite(idx, idx + offset);
                        ds.unite(idx, idx + 2 * offset);
                        ds.unite(idx, idx + 3 * offset);
                    }
                }
            }
        }

        (0..4 * n * n)
            .map(|i| ds.root(i))
            .collect::<HashSet<_>>()
            .len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0959() {
        assert_eq!(
            Solution::regions_by_slashes(vec![" /".to_string(), "/ ".to_string()]),
            2
        );
        assert_eq!(
            Solution::regions_by_slashes(vec![" /".to_string(), "  ".to_string()]),
            1
        );
        assert_eq!(
            Solution::regions_by_slashes(vec!["\\/".to_string(), "/\\".to_string()]),
            4
        );
        assert_eq!(
            Solution::regions_by_slashes(vec!["/\\".to_string(), "\\/".to_string()]),
            5
        );
        assert_eq!(
            Solution::regions_by_slashes(vec!["//".to_string(), "/ ".to_string()]),
            3
        );
    }
}
