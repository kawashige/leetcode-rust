pub struct Solution {}

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let threshold = threshold * k;
        let mut sum_k = (0..k as usize).map(|i| arr[i]).sum();
        let mut result = if threshold <= sum_k { 1 } else { 0 };

        for i in (k as usize)..arr.len() {
            sum_k += arr[i] - arr[i - k as usize];
            if threshold <= sum_k {
                result += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1343() {
        assert_eq!(
            Solution::num_of_subarrays(vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4),
            3
        );
        assert_eq!(
            Solution::num_of_subarrays(vec![11, 13, 17, 23, 29, 31, 7, 5, 2, 3], 3, 5),
            6
        );
    }
}
