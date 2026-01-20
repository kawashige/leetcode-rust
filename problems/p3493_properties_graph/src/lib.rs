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
    pub fn number_of_components(properties: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut integers = vec![vec![false; 101]; properties.len()];
        for i in 0..integers.len() {
            for j in 0..properties[i].len() {
                integers[i][properties[i][j] as usize] = true;
            }
        }

        let mut ds = DisjointSet::new(integers.len());
        for i in 0..integers.len() {
            for j in 0..i {
                let mut common = 0;
                for l in 1..101 {
                    if integers[i][l] && integers[j][l] {
                        common += 1;
                    }
                }
                if k <= common {
                    ds.unite(i, j);
                }
            }
        }

        (0..integers.len())
            .map(|i| ds.root(i))
            .collect::<HashSet<_>>()
            .len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3493() {
        assert_eq!(
            Solution::number_of_components(
                vec![
                    vec![1, 2],
                    vec![1, 1],
                    vec![3, 4],
                    vec![4, 5],
                    vec![5, 6],
                    vec![7, 7]
                ],
                1
            ),
            3
        );
        assert_eq!(
            Solution::number_of_components(vec![vec![1, 2, 3], vec![2, 3, 4], vec![4, 3, 5]], 2),
            1
        );
        assert_eq!(
            Solution::number_of_components(vec![vec![1, 1], vec![1, 1]], 2),
            2
        );
    }
}
