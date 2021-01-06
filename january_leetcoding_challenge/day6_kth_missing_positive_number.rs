pub struct Solution {}

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        (0..arr.len() as i32)
            .find(|i| k <= arr[*i as usize] - *i - 1)
            .unwrap_or(arr.len() as i32)
            + k
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_dya06() {
        assert_eq!(9, Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 5));
        assert_eq!(6, Solution::find_kth_positive(vec![1, 2, 3, 4], 2));
        assert_eq!(1, Solution::find_kth_positive(vec![2], 1));
        assert_eq!(20, Solution::find_kth_positive(vec![1, 13, 18], 17));
    }
}
