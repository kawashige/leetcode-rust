pub struct Solution {}

impl Solution {
    pub fn get_strongest(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        arr.sort_unstable();
        let m = arr[(arr.len() - 1) / 2];
        let mut result = Vec::with_capacity(k as usize);

        let mut l = 0;
        let mut r = arr.len() - 1;

        while result.len() < k as usize {
            if (arr[r] - m).abs() < (arr[l] - m).abs() {
                result.push(arr[l]);
                l += 1;
            } else {
                result.push(arr[r]);
                r -= 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1471() {
        assert_eq!(Solution::get_strongest(vec![1, 2, 3, 4, 5], 2), vec![5, 1]);
        assert_eq!(Solution::get_strongest(vec![1, 1, 3, 5, 5], 2), vec![5, 5]);
        assert_eq!(
            Solution::get_strongest(vec![6, 7, 11, 7, 6, 8], 5),
            vec![11, 8, 6, 6, 7]
        );
    }
}
