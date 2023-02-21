pub struct Solution {}

impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        triplets
            .into_iter()
            .filter(|t| (0..t.len()).all(|i| t[i] <= target[i]))
            .fold(vec![0, 0, 0], |acc, t| {
                (0..acc.len()).map(|i| acc[i].max(t[i])).collect::<Vec<_>>()
            })
            == target
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1899() {
        assert!(Solution::merge_triplets(
            vec![vec![2, 5, 3], vec![1, 8, 4], vec![1, 7, 5]],
            vec![2, 7, 5]
        ));
        assert!(!Solution::merge_triplets(
            vec![vec![3, 4, 5], vec![4, 5, 6]],
            vec![3, 2, 5]
        ));
        assert!(Solution::merge_triplets(
            vec![vec![2, 5, 3], vec![2, 3, 4], vec![1, 2, 5], vec![5, 2, 3]],
            vec![5, 5, 5]
        ));
    }
}
