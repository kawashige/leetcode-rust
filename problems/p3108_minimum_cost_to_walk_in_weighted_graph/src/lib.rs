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
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let mut cost = vec![-1; n as usize];
        let mut ds = DisjointSet::new(n as usize);

        for edge in edges {
            let mut c = edge[2];
            for i in 0..2 {
                if cost[ds.root(edge[i] as usize)] != -1 {
                    c &= cost[ds.root(edge[i] as usize)];
                }
            }
            ds.unite(edge[0] as usize, edge[1] as usize);
            cost[ds.root(edge[0] as usize)] = c;
        }

        query
            .into_iter()
            .map(|q| {
                if ds.root(q[0] as usize) == ds.root(q[1] as usize) {
                    cost[ds.root(q[0] as usize)]
                } else {
                    -1
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3108() {
        assert_eq!(
            Solution::minimum_cost(
                5,
                vec![vec![0, 1, 7], vec![1, 3, 7], vec![1, 2, 1]],
                vec![vec![0, 3], vec![3, 4]]
            ),
            vec![1, -1]
        );
        assert_eq!(
            Solution::minimum_cost(
                3,
                vec![vec![0, 2, 7], vec![0, 1, 15], vec![1, 2, 6], vec![1, 2, 1]],
                vec![vec![1, 2]]
            ),
            vec![0]
        );
    }
}
