use std::collections::{HashMap, VecDeque};

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
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let c = c as usize;
        let mut ds = DisjointSet::new(c);
        for connection in connections {
            ds.unite(connection[0] as usize - 1, connection[1] as usize - 1);
        }

        let mut parent = vec![0; c];
        for i in 0..c {
            parent[i] = ds.root(i);
        }

        let mut group = HashMap::new();
        for i in 0..c {
            (*group.entry(parent[i]).or_insert(VecDeque::new())).push_back(i);
        }

        let mut is_online = vec![true; c];
        let mut result = Vec::new();
        for q in queries {
            if q[0] == 1 {
                if is_online[q[1] as usize - 1] {
                    result.push(q[1]);
                } else {
                    let mut j = -1;
                    while let Some(k) = group
                        .get_mut(&parent[q[1] as usize - 1])
                        .unwrap()
                        .pop_front()
                    {
                        if is_online[k] {
                            j = k as i32 + 1;
                            group
                                .get_mut(&parent[q[1] as usize - 1])
                                .unwrap()
                                .push_front(k);
                            break;
                        }
                    }
                    result.push(j);
                }
            } else {
                is_online[q[1] as usize - 1] = false;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3607() {
        assert_eq!(
            Solution::process_queries(
                5,
                vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]],
                vec![vec![1, 3], vec![2, 1], vec![1, 1], vec![2, 2], vec![1, 2]]
            ),
            vec![3, 2, 3]
        );
        assert_eq!(
            Solution::process_queries(3, vec![], vec![vec![1, 1], vec![2, 1], vec![1, 1]]),
            vec![1, -1]
        );
    }
}
