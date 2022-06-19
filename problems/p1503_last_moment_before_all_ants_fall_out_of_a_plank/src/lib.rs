pub struct Solution {}

impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        (*left.iter().max().unwrap_or(&0)).max(n - right.iter().min().unwrap_or(&n))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1503() {
        assert_eq!(Solution::get_last_moment(4, vec![4,3], vec![0, 1]), 4);
        assert_eq!(Solution::get_last_moment(7, vec![], vec![0, 1,2,3,4,5,6,7]), 7);
        assert_eq!(Solution::get_last_moment(7, vec![0, 1,2,3,4,5,6,7], vec![]), 7);
    }
}