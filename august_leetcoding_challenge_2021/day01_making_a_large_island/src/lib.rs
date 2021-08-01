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
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut disjoint_set = DisjointSet::new(n * n);

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 0 {
                    continue;
                }
                if i > 0 && grid[i - 1][j] == 1 {
                    disjoint_set.unite(i * n + j, (i - 1) * n + j);
                }
                if j > 0 && grid[i][j - 1] == 1 {
                    disjoint_set.unite(i * n + j, i * n + j - 1);
                }
            }
        }

        let mut max = 0;
        for i in 0..n {
            for j in 0..n {
                let area = if grid[i][j] == 1 {
                    disjoint_set.size(i * n + j)
                } else {
                    let mut root = vec![];
                    let mut area = 1;
                    for (r, c) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                        let (x, y) = (i as i32 + r, j as i32 + c);
                        if x < 0
                            || n as i32 <= x
                            || y < 0
                            || n as i32 <= y
                            || grid[x as usize][y as usize] == 0
                            || root.contains(&disjoint_set.root(x as usize * n + y as usize))
                        {
                            continue;
                        }
                        root.push(disjoint_set.root(x as usize * n + y as usize));
                        area += disjoint_set.size(x as usize * n + y as usize);
                    }
                    area
                };
                max = max.max(area);
            }
        }

        max as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day01() {
        assert_eq!(Solution::largest_island(vec![vec![0, 0], vec![0, 0]]), 1);
        assert_eq!(Solution::largest_island(vec![vec![1, 0], vec![0, 1]]), 3);
        assert_eq!(Solution::largest_island(vec![vec![1, 1], vec![1, 0]]), 4);
        assert_eq!(Solution::largest_island(vec![vec![1, 1], vec![1, 1]]), 4);
    }
}
