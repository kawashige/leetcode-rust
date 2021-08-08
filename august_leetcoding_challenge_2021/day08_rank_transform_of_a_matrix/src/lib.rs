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
    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut disjoint_set = DisjointSet::new(matrix.len() * matrix[0].len());
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                for k in (i + 1)..matrix.len() {
                    if matrix[i][j] == matrix[k][j] {
                        disjoint_set.unite(i * matrix[0].len() + j, k * matrix[0].len() + j);
                        break;
                    }
                }
                for k in (j + 1)..matrix[0].len() {
                    if matrix[i][j] == matrix[i][k] {
                        disjoint_set.unite(i * matrix[0].len() + j, i * matrix[0].len() + k);
                        break;
                    }
                }
            }
        }

        let mut values = Vec::with_capacity(matrix.len() * matrix[0].len());
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                values.push((
                    matrix[i][j],
                    disjoint_set.root(i * matrix[0].len() + j),
                    (i, j),
                ));
            }
        }
        values.sort_unstable();

        let mut result = vec![vec![0; matrix[0].len()]; matrix.len()];
        let mut column_min = vec![0; matrix.len()];
        let mut row_min = vec![0; matrix[0].len()];

        let mut i = 0;
        while i < values.len() {
            let mut target = vec![values[i].2];
            while i + 1 < values.len()
                && values[i + 1].0 == values[i].0
                && values[i + 1].1 == values[i].1
            {
                i += 1;
                target.push(values[i].2);
            }

            let rank = target
                .iter()
                .map(|(c, r)| column_min[*c].max(row_min[*r]) + 1)
                .max()
                .unwrap();

            for (c, r) in target {
                column_min[c] = rank;
                row_min[r] = rank;
                result[c][r] = rank;
            }
            i += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day08() {
        assert_eq!(
            Solution::matrix_rank_transform(vec![
                vec![-24, -9, -14, -15, 44, 31, -46, 5, 20, -5, 34],
                vec![9, -40, -49, -50, 17, 40, 35, 30, -39, 36, -49],
                vec![-18, -43, -40, -5, -30, 9, -28, -41, -6, -47, 12],
                vec![11, 42, -23, 20, 35, 34, -39, -16, 27, 34, -15],
                vec![32, 27, -30, 29, -48, 15, -50, -47, -28, -21, 38],
                vec![45, 48, -1, -18, 9, -4, -13, 10, 9, 8, -41],
                vec![-42, -35, 20, -17, 10, 5, 36, 47, 6, 1, 8],
                vec![3, -50, -23, 16, 31, 2, -39, 36, -25, -30, 37],
                vec![-48, -41, 18, -31, -48, -1, -42, -3, -8, -29, -2],
                vec![17, 0, 31, -30, -43, -20, -37, -6, -43, 8, 19],
                vec![42, 25, 32, 27, -2, 45, 12, -9, 34, 17, 32]
            ]),
            vec![
                vec![4, 11, 10, 9, 25, 21, 2, 14, 20, 12, 24],
                vec![18, 5, 2, 1, 21, 25, 23, 22, 6, 24, 2],
                vec![8, 2, 5, 11, 6, 18, 7, 4, 10, 1, 20],
                vec![19, 24, 9, 20, 23, 22, 4, 10, 21, 22, 11],
                vec![23, 20, 6, 22, 2, 19, 1, 3, 7, 8, 26],
                vec![26, 27, 11, 7, 19, 9, 8, 20, 19, 14, 3],
                vec![3, 6, 21, 8, 20, 17, 24, 25, 18, 13, 19],
                vec![17, 1, 9, 18, 22, 16, 4, 23, 8, 5, 25],
                vec![2, 4, 16, 5, 2, 15, 3, 13, 9, 6, 14],
                vec![20, 13, 22, 6, 3, 7, 5, 12, 3, 14, 21],
                vec![25, 16, 23, 21, 12, 26, 13, 11, 24, 15, 23]
            ]
        );
        assert_eq!(
            Solution::matrix_rank_transform(vec![
                vec![-37, -50, -3, 44],
                vec![-37, 46, 13, -32],
                vec![47, -42, -3, -40],
                vec![-17, -22, -39, 24]
            ]),
            vec![
                vec![2, 1, 4, 6],
                vec![2, 6, 5, 4],
                vec![5, 2, 4, 3],
                vec![4, 3, 1, 5]
            ]
        );
        assert_eq!(
            Solution::matrix_rank_transform(vec![vec![1, 2], vec![3, 4]]),
            vec![vec![1, 2], vec![2, 3]]
        );
        assert_eq!(
            Solution::matrix_rank_transform(vec![vec![7, 7], vec![7, 7]]),
            vec![vec![1, 1], vec![1, 1]]
        );
        assert_eq!(
            Solution::matrix_rank_transform(vec![
                vec![20, -21, 14],
                vec![-19, 4, 19],
                vec![22, -47, 24],
                vec![-19, 4, 19]
            ]),
            vec![vec![4, 2, 3], vec![1, 3, 4], vec![5, 1, 6], vec![1, 3, 4]]
        );
        assert_eq!(
            Solution::matrix_rank_transform(vec![vec![7, 3, 6], vec![1, 4, 5], vec![9, 8, 2]]),
            vec![vec![5, 1, 4], vec![1, 2, 3], vec![6, 3, 1]]
        );
    }
}
