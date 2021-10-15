pub struct Solution {}

impl Solution {
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        points.sort_unstable_by_key(|v| v[0] * v[0] + v[1] * v[1]);
        points.into_iter().take(k as usize).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0973() {
        assert_eq!(
            Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1),
            vec![vec![-2, 2]]
        );
        assert_eq!(
            Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2),
            vec![vec![3, 3], vec![-2, 4]]
        );
    }
}
