use std::collections::BinaryHeap;

pub struct Solution {}

#[derive(Clone)]
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
    pub fn is_ok(
        mid: i32,
        n: i32,
        k: i32,
        heap: &BinaryHeap<(i32, usize, usize)>,
        ds: &DisjointSet,
    ) -> bool {
        let mut heap = heap.clone();
        let mut ds = ds.clone();
        let mut k = k;
        let mut n = n;
        while let Some((s, i, j)) = heap.pop() {
            if ds.root(i) == ds.root(j) {
                continue;
            }
            if s < mid {
                if k == 0 || s * 2 < mid {
                    return false;
                }
                k -= 1;
            }
            ds.unite(i, j);
            n -= 1;
        }
        n == 0
    }

    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut ds = DisjointSet::new(n as usize);
        let mut heap = BinaryHeap::new();
        let mut min_strength = 200_001;
        let mut size = 0;
        for e in edges {
            if e[3] == 1 {
                if ds.root(e[0] as usize) == ds.root(e[1] as usize) {
                    return -1;
                }
                ds.unite(e[0] as usize, e[1] as usize);
                min_strength = min_strength.min(e[2]);
                size += 1;
            } else {
                heap.push((e[2], e[0] as usize, e[1] as usize));
            }
        }

        if !Self::is_ok(0, n - 1 - size, k, &heap, &ds) {
            return -1;
        }

        let mut ok = 0;
        let mut ng = min_strength + 1;

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, n - 1 - size, k, &heap, &ds) {
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
    fn test_3600() {
        assert_eq!(
            Solution::max_stability(3, vec![vec![0, 1, 2, 1], vec![1, 2, 3, 0]], 1),
            2
        );
        assert_eq!(
            Solution::max_stability(
                3,
                vec![vec![0, 1, 4, 0], vec![1, 2, 3, 0], vec![0, 2, 1, 0]],
                2
            ),
            6
        );
        assert_eq!(
            Solution::max_stability(
                3,
                vec![vec![0, 1, 1, 1], vec![1, 2, 1, 1], vec![2, 0, 1, 1]],
                0
            ),
            -1
        );
    }
}
