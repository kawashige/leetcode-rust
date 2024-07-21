pub struct Solution {}

impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut incount = vec![0; k as usize + 1];
        let mut next = vec![vec![]; k as usize + 1];
        for cond in row_conditions {
            incount[cond[1] as usize] += 1;
            next[cond[0] as usize].push(cond[1] as usize);
        }
        let mut stack = (1..incount.len())
            .filter(|i| incount[*i] == 0)
            .collect::<Vec<_>>();
        let mut row_order = vec![0; k as usize + 1];
        let mut ord = 0;
        while let Some(i) = stack.pop() {
            row_order[i] = ord;
            ord += 1;
            for n in &next[i] {
                incount[*n] -= 1;
                if incount[*n] == 0 {
                    stack.push(*n);
                }
            }
        }
        if ord != k as usize {
            return Default::default();
        }

        let mut incount = vec![0; k as usize + 1];
        let mut next = vec![vec![]; k as usize + 1];
        for cond in col_conditions {
            incount[cond[1] as usize] += 1;
            next[cond[0] as usize].push(cond[1] as usize);
        }
        let mut stack = (1..incount.len())
            .filter(|i| incount[*i] == 0)
            .collect::<Vec<_>>();
        let mut col_order = vec![0; k as usize + 1];
        let mut ord = 0;
        while let Some(i) = stack.pop() {
            col_order[i] = ord;
            ord += 1;
            for n in &next[i] {
                incount[*n] -= 1;
                if incount[*n] == 0 {
                    stack.push(*n);
                }
            }
        }
        if ord != k as usize {
            return Default::default();
        }

        let mut result = vec![vec![0; k as usize]; k as usize];
        for i in 1..=k as usize {
            result[row_order[i]][col_order[i]] = i as i32;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2392() {
        assert_eq!(
            Solution::build_matrix(
                3,
                vec![vec![1, 2], vec![3, 2]],
                vec![vec![2, 1], vec![3, 2]]
            ),
            vec![vec![3, 0, 0], vec![0, 0, 1], vec![0, 2, 0]]
        );
        assert_eq!(
            Solution::build_matrix(
                3,
                vec![vec![1, 2], vec![2, 3], vec![3, 1], vec![2, 3]],
                vec![vec![2, 1]]
            ),
            vec![] as Vec<Vec<i32>>
        );
    }
}
