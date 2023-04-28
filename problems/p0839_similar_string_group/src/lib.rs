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
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let mut disjoint_set = DisjointSet::new(strs.len());

        for i in 0..strs.len() {
            for j in (i + 1)..strs.len() {
                if (0..strs[i].len())
                    .filter(|k| strs[i].as_bytes()[*k] != strs[j].as_bytes()[*k])
                    .count()
                    <= 2
                {
                    disjoint_set.unite(i, j);
                }
            }
        }

        (0..strs.len())
            .map(|i| disjoint_set.root(i))
            .collect::<HashSet<_>>()
            .len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0839() {
        assert_eq!(
            Solution::num_similar_groups(vec![
                "tars".to_string(),
                "rats".to_string(),
                "arts".to_string(),
                "star".to_string()
            ]),
            2
        );
        assert_eq!(
            Solution::num_similar_groups(vec!["omv".to_string(), "ovm".to_string()]),
            1
        );
    }
}
