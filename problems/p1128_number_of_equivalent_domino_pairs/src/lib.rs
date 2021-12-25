pub struct Solution {}

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut count = vec![vec![0; 10]; 10];
        let mut result = 0;

        for i in (0..dominoes.len()).rev() {
            let min = dominoes[i][0].min(dominoes[i][1]) as usize;
            let max = dominoes[i][0].max(dominoes[i][1]) as usize;
            result += count[min][max];
            count[min][max] += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1128() {
        assert_eq!(
            Solution::num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]]),
            1
        );
        assert_eq!(
            Solution::num_equiv_domino_pairs(vec![
                vec![1, 2],
                vec![1, 2],
                vec![1, 1],
                vec![1, 2],
                vec![2, 2]
            ]),
            3
        );
    }
}
