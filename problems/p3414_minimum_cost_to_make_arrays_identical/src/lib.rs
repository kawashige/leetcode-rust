pub struct Solution {}

impl Solution {
    pub fn min_cost(arr: Vec<i32>, brr: Vec<i32>, k: i64) -> i64 {
        let result = (0..arr.len())
            .map(|i| (arr[i] - brr[i]).abs() as i64)
            .sum::<i64>();

        let mut arr = arr;
        let mut brr = brr;
        arr.sort_unstable();
        brr.sort_unstable();

        result.min(
            k + (0..arr.len())
                .map(|i| (arr[i] - brr[i]).abs() as i64)
                .sum::<i64>(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3414() {
        assert_eq!(Solution::min_cost(vec![-7, 9, 5], vec![7, -2, -5], 2), 13);
        assert_eq!(Solution::min_cost(vec![2, 1], vec![2, 1], 0), 0);
    }
}
