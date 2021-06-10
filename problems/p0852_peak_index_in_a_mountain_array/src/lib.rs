pub struct Solution {}

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        (1..(arr.len() - 1))
            .find(|i| arr[i - 1] < arr[*i] && arr[*i] > arr[i + 1])
            .unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0852() {
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
        assert_eq!(Solution::peak_index_in_mountain_array(vec![3, 4, 5, 1]), 2);
        assert_eq!(
            Solution::peak_index_in_mountain_array(vec![24, 69, 100, 99, 79, 78, 67, 36, 26, 19]),
            2
        );
    }
}
