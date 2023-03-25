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
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut disjoint_set = DisjointSet::new(n as usize);
        for edge in edges {
            disjoint_set.unite(edge[0] as usize, edge[1] as usize);
        }

        (0..n)
            .map(|i| (n as usize - disjoint_set.size(i as usize)) as i64)
            .sum::<i64>()
            / 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2316() {
        assert_eq!(
            Solution::count_pairs(3, vec![vec![0, 1], vec![0, 2], vec![1, 2]]),
            0
        );
        assert_eq!(
            Solution::count_pairs(
                7,
                vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]]
            ),
            14
        );
    }
}
