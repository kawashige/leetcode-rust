use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn recurse(
        i: usize,
        j: usize,
        di: i32,
        dj: i32,
        v: i32,
        mat: &Vec<Vec<i32>>,
        is_prime: &Vec<bool>,
        freq: &mut HashMap<i32, i32>,
    ) {
        let new_v = v * 10 + mat[i][j];
        if 10 < new_v && is_prime[new_v as usize] {
            *freq.entry(new_v).or_insert(0) += 1;
        }
        let (new_i, new_j) = (i as i32 + di, j as i32 + dj);
        if !(0..mat.len() as i32).contains(&new_i) || !(0..mat[0].len() as i32).contains(&new_j) {
            return;
        }
        Self::recurse(
            new_i as usize,
            new_j as usize,
            di,
            dj,
            new_v,
            mat,
            is_prime,
            freq,
        );
    }

    pub fn most_frequent_prime(mat: Vec<Vec<i32>>) -> i32 {
        let mut is_prime = vec![true; 10_usize.pow(mat.len().max(mat[0].len()) as u32)];
        for i in 2..is_prime.len() {
            if !is_prime[i] {
                continue;
            }
            for j in ((i + i)..is_prime.len()).step_by(i) {
                is_prime[j] = false;
            }
        }

        let mut freq = HashMap::new();

        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                for (di, dj) in [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ]
                .iter()
                {
                    Self::recurse(i, j, *di, *dj, 0, &mat, &is_prime, &mut freq);
                }
            }
        }

        if freq.is_empty() {
            return -1;
        }

        let mut freq = freq.into_iter().collect::<Vec<_>>();
        freq.sort_unstable_by_key(|x| (x.1, x.0));
        freq.last().unwrap().0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3044() {
        assert_eq!(
            Solution::most_frequent_prime(vec![vec![1, 1], vec![9, 9], vec![1, 1]]),
            19
        );
        assert_eq!(Solution::most_frequent_prime(vec![vec![7]]), -1);
        assert_eq!(
            Solution::most_frequent_prime(vec![vec![9, 7, 8], vec![4, 6, 5], vec![2, 8, 6]]),
            97
        );
    }
}
