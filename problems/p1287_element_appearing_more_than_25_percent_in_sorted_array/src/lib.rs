pub struct Solution {}

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let threshold = arr.len() / 4;
        let mut count = 0;
        for i in 0..arr.len() {
            if i == 0 || arr[i - 1] == arr[i] {
                count += 1;
            } else {
                count = 1;
            }
            if threshold < count {
                return arr[i];
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1287() {
        assert_eq!(
            Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]),
            6
        );
        assert_eq!(Solution::find_special_integer(vec![1, 1]), 1);
        assert_eq!(Solution::find_special_integer(vec![1]), 1);
        assert_eq!(Solution::find_special_integer(vec![1, 2, 3, 3]), 3);
    }
}
