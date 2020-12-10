pub struct Solution {}

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        let mut i = 0;
        while i < arr.len() - 1 && arr[i] < arr[i + 1] {
            i += 1;
        }
        if i < 1 || i == arr.len() - 1 || arr[i] == arr[i + 1] {
            return false;
        }
        while i < arr.len() - 1 && arr[i] > arr[i + 1] {
            i += 1;
        }
        i == arr.len() - 1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day10() {
        assert!(!Solution::valid_mountain_array(vec![2, 1]));
        assert!(!Solution::valid_mountain_array(vec![2, 5, 5]));
        assert!(Solution::valid_mountain_array(vec![0, 3, 2, 1]));
        assert!(!Solution::valid_mountain_array(vec![2, 3, 5]));
        assert!(!Solution::valid_mountain_array(vec![9, 7, 5]));
    }
}
