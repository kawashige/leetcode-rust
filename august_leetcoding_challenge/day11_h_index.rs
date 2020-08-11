pub struct Solution {}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut sorted = citations.clone();
        sorted.sort();
        sorted.reverse();
        for (i, c) in (0..sorted.len()).zip(sorted.iter()) {
            if *c == 0 {
                return i as i32;
            } else if (i + 1) as i32 >= *c {
                return std::cmp::max(*c, i as i32);
            }
        }
        sorted.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day11() {
        assert_eq!(3, Solution::h_index(vec![3, 0, 6, 1, 5]));
        assert_eq!(4, Solution::h_index(vec![10, 8, 5, 4, 3]));
        assert_eq!(3, Solution::h_index(vec![3, 3, 3, 3, 3]));
        assert_eq!(1, Solution::h_index(vec![100]));
        assert_eq!(0, Solution::h_index(vec![]));
        assert_eq!(2, Solution::h_index(vec![11, 14]));
        assert_eq!(2, Solution::h_index(vec![4, 4, 0, 0]));
        assert_eq!(3, Solution::h_index(vec![1, 7, 9, 4]));
        assert_eq!(2, Solution::h_index(vec![0, 0, 9, 9]));
    }
}
