pub struct Solution {}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let l = citations.len() as i32;
        l - (0..l).find(|i| l - i <= citations[*i as usize]).unwrap_or(l)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0275() {
        assert_eq!(3, Solution::h_index(vec![0,1,3,5,6]));
        assert_eq!(3, Solution::h_index(vec![0, 1, 3, 5, 6]));
        assert_eq!(4, Solution::h_index(vec![3,4,5,8,10]));
        assert_eq!(3, Solution::h_index(vec![3, 3, 3, 3, 3]));
        assert_eq!(1, Solution::h_index(vec![100]));
        assert_eq!(0, Solution::h_index(vec![]));
    }
}
