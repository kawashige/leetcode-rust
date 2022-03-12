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
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut ds = DisjointSet::new(n as usize);
        let mut count = 0;

        for connection in connections {
            if ds.root(connection[0] as usize) == ds.root(connection[1] as usize) {
                count += 1;
            } else {
                ds.unite(connection[0] as usize, connection[1] as usize);
            }
        }

        let islands = (0..n)
            .map(|i| ds.root(i as usize))
            .collect::<HashSet<_>>()
            .len() as i32;

        if 1 < islands && count < islands - 1 {
            -1
        } else {
            islands - 1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1319() {
        assert_eq!(
            Solution::make_connected(4, vec![vec![0, 1], vec![0, 2], vec![1, 2]]),
            1
        );
        assert_eq!(
            Solution::make_connected(
                6,
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]]
            ),
            2
        );
        assert_eq!(
            Solution::make_connected(6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]]),
            -1
        );
    }
}
