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
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        let mut disjoint_set = DisjointSet::new(nums.len());
        let mut indices = vec![nums.len(); 100_001];

        for i in 0..nums.len() {
            let mut num = nums[i] as usize;
            for p in 2..=num {
                if num < p * p {
                    break;
                }
                if num % p != 0 {
                    continue;
                }

                while num % p == 0 {
                    num /= p;
                }

                if indices[p] != nums.len() {
                    disjoint_set.unite(i, indices[p]);
                } else {
                    indices[p] = i;
                }
            }

            if num != 1 {
                if indices[num] != nums.len() {
                    disjoint_set.unite(i, indices[num]);
                } else {
                    indices[num] = i;
                }
            }
        }

        disjoint_set.size(0) == nums.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2709() {
        assert!(Solution::can_traverse_all_pairs(vec![10007, 20014]));
        assert!(Solution::can_traverse_all_pairs(vec![2, 3, 6]));
        assert!(!Solution::can_traverse_all_pairs(vec![3, 9, 5]));
        assert!(Solution::can_traverse_all_pairs(vec![4, 3, 12, 8]));
    }
}
