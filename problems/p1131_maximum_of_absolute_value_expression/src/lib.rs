pub struct Solution {}

impl Solution {
    pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut max1 = -arr1[0] - arr2[0];
        let mut max2 = -arr1[0] + arr2[0];
        let mut max3 = arr1[0] - arr2[0];
        let mut max4 = arr1[0] + arr2[0];
        let mut result = std::i32::MIN;

        for i in 1..arr1.len() {
            result = result.max(arr1[i] + arr2[i] + i as i32 + max1);
            result = result.max(arr1[i] - arr2[i] + i as i32 + max2);
            result = result.max(-arr1[i] + arr2[i] + i as i32 + max3);
            result = result.max(-arr1[i] - arr2[i] + i as i32 + max4);
            max1 = max1.max(-arr1[i] - arr2[i] - i as i32);
            max2 = max2.max(-arr1[i] + arr2[i] - i as i32);
            max3 = max3.max(arr1[i] - arr2[i] - i as i32);
            max4 = max4.max(arr1[i] + arr2[i] - i as i32);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1131() {
        assert_eq!(
            Solution::max_abs_val_expr(vec![1, 2, 3, 4], vec![-1, 4, 5, 6]),
            13
        );
        assert_eq!(
            Solution::max_abs_val_expr(vec![1, -2, -5, 0, 10], vec![0, -2, -1, -7, -4]),
            20
        );
    }
}
