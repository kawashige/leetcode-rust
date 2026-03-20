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
    pub fn is_ok(mid: i32, k: usize, n: usize, edges: &Vec<Vec<i32>>) -> bool {
        let mut ds = DisjointSet::new(n);
        let mut count = n;

        for e in edges {
            if mid < e[2] {
                continue;
            }
            if ds.root(e[0] as usize) != ds.root(e[1] as usize) {
                count -= 1;
                ds.unite(e[0] as usize, e[1] as usize);
            }
        }

        count <= k
    }

    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        if k == n {
            return 0;
        }
        let mut ng = 0;
        let mut ok = edges.iter().map(|e| e[2]).max().unwrap();

        while ng + 1 < ok {
            let mid = (ng + ok) / 2;
            if Self::is_ok(mid, k as usize, n as usize, &edges) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3613() {
        assert_eq!(
            Solution::min_cost(
                5,
                vec![vec![0, 1, 4], vec![1, 2, 3], vec![1, 3, 2], vec![3, 4, 6]],
                2
            ),
            4
        );
        assert_eq!(
            Solution::min_cost(4, vec![vec![0, 1, 5], vec![1, 2, 5], vec![2, 3, 5]], 1),
            5
        );
    }
}
