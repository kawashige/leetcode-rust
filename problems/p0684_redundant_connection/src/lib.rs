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

        if self.size(parent_i) > self.size(parent_j) {
            std::mem::swap(&mut parent_i, &mut parent_j);
        }

        self.parent[parent_j] = parent_i;
        self.size[parent_i] += self.size[parent_j];
    }

    fn size(&mut self, i: usize) -> usize {
        let root = self.root(i);
        self.size[root]
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut disjoint_set = DisjointSet::new(edges.len());
        for edge in edges {
            let (f, t) = (edge[0] as usize - 1, edge[1] as usize - 1);
            if disjoint_set.root(f) == disjoint_set.root(t) {
                return edge;
            }
            disjoint_set.unite(f, t);
        }

        vec![0, 0]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0684() {
        assert_eq!(
            Solution::find_redundant_connection(vec![
                vec![9, 10],
                vec![5, 8],
                vec![2, 6],
                vec![1, 5],
                vec![3, 8],
                vec![4, 9],
                vec![8, 10],
                vec![4, 10],
                vec![6, 8],
                vec![7, 9]
            ]),
            vec![4, 10]
        );
        assert_eq!(
            Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            vec![2, 3]
        );
        assert_eq!(
            Solution::find_redundant_connection(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![1, 4],
                vec![1, 5]
            ]),
            vec![1, 4]
        );
    }
}
