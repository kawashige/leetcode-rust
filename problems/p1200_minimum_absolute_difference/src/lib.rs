pub struct Solution {}

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();
        let min = (1..arr.len()).map(|i| arr[i] - arr[i - 1]).min().unwrap();
        (1..arr.len())
            .filter(|i| arr[*i] - arr[i - 1] == min)
            .map(|i| vec![arr[i - 1], arr[i]])
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1200() {
        assert_eq!(
            Solution::minimum_abs_difference(vec![4, 2, 1, 3]),
            vec![vec![1, 2], vec![2, 3], vec![3, 4]]
        );
        assert_eq!(
            Solution::minimum_abs_difference(vec![1, 3, 6, 10, 15]),
            vec![vec![1, 3]]
        );
        assert_eq!(
            Solution::minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27]),
            vec![vec![-14, -10], vec![19, 23], vec![23, 27]]
        );
    }
}
