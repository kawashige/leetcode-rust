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
    pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        const M: i64 = 1_000_000_007;

        let mut disjoint_set = DisjointSet::new(nums.len());
        let mut num_indices = nums.iter().cloned().enumerate().collect::<Vec<_>>();
        num_indices.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        let mut max_min_product = 0;
        let mut sums = nums
            .iter()
            .cloned()
            .map(|num| num as i64)
            .collect::<Vec<_>>();

        for (index, num) in num_indices {
            let mut sum = sums[disjoint_set.root(index)];
            if 0 < index && nums[index] < nums[index - 1] {
                sum += sums[disjoint_set.root(index - 1)];
                disjoint_set.unite(index, index - 1);
            }
            if index + 1 < nums.len() && nums[index] <= nums[index + 1] {
                sum += sums[disjoint_set.root(index + 1)];
                disjoint_set.unite(index, index + 1);
            }
            sums[disjoint_set.root(index)] = sum;
            max_min_product = max_min_product.max(sum * num as i64);
        }

        (max_min_product % M) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1856() {
        assert_eq!(
            Solution::max_sum_min_product(vec![3, 3, 2, 2, 3, 1, 1, 4, 1, 3]),
            26
        );
        assert_eq!(
            Solution::max_sum_min_product(vec![2, 5, 4, 2, 4, 5, 3, 1, 2, 4]),
            50
        );
        assert_eq!(Solution::max_sum_min_product(vec![1, 2, 3, 2]), 14);
        assert_eq!(Solution::max_sum_min_product(vec![2, 3, 3, 1, 2]), 18);
        assert_eq!(Solution::max_sum_min_product(vec![3, 1, 5, 6, 4, 2]), 60);
    }
}
