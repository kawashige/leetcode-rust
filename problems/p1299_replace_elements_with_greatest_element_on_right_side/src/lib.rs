pub struct Solution {}

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut result = vec![-1; arr.len()];
        let mut max = *arr.last().unwrap();

        for i in (0..arr.len() - 1).rev() {
            result[i] = max;
            max = max.max(arr[i]);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1299() {
        assert_eq!(
            Solution::replace_elements(vec![17, 18, 5, 4, 6, 1]),
            vec![18, 6, 6, 6, 1, -1]
        );
        assert_eq!(Solution::replace_elements(vec![400]), vec![-1]);
    }
}
