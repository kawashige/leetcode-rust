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
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut ds = DisjointSet::new(n as usize);
        for i in 1..nums.len() {
            if nums[i] - nums[i - 1] <= max_diff {
                ds.unite(i, i - 1);
            }
        }

        queries
            .into_iter()
            .map(|q| ds.root(q[0] as usize) == ds.root(q[1] as usize))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3532() {
        assert_eq!(
            Solution::path_existence_queries(2, vec![1, 3], 1, vec![vec![0, 0], vec![0, 1]]),
            vec![true, false]
        );
        assert_eq!(
            Solution::path_existence_queries(
                4,
                vec![2, 5, 6, 8],
                2,
                vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]]
            ),
            vec![false, false, true, true]
        );
    }
}
