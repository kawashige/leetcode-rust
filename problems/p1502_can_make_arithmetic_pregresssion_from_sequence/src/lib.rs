pub struct Solution {}

impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();
        let target = arr[1] - arr[0];
        (2..arr.len()).all(|i| arr[i] - arr[i - 1] == target)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1502() {
        assert!(Solution::can_make_arithmetic_progression(vec![3, 5, 1]));
        assert!(!Solution::can_make_arithmetic_progression(vec![1, 2, 4]));
    }
}
