pub struct Solution {}

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            vec![0; code.len()]
        } else if 0 < k {
            (0..code.len())
                .map(|i| (1..=k as usize).map(|j| code[(i + j) % code.len()]).sum())
                .collect()
        } else {
            (0..code.len())
                .map(|i| {
                    (1..=k.abs() as usize)
                        .map(|j| code[(i + code.len() - j) % code.len()])
                        .sum()
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1652() {
        assert_eq!(Solution::decrypt(vec![5, 7, 1, 4], 3), vec![12, 10, 16, 13]);
        assert_eq!(Solution::decrypt(vec![1, 2, 3, 4], 0), vec![0, 0, 0, 0]);
        assert_eq!(Solution::decrypt(vec![2, 4, 9, 3], -2), vec![12, 5, 6, 13]);
    }
}
