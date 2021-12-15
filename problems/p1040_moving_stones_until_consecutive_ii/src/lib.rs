pub struct Solution {}

impl Solution {
    pub fn num_moves_stones_ii(mut stones: Vec<i32>) -> Vec<i32> {
        stones.sort_unstable();
        stones
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1040() {
        assert_eq!(Solution::num_moves_stones_ii(vec![7, 4, 9]), vec![1, 2]);
        assert_eq!(
            Solution::num_moves_stones_ii(vec![6, 5, 4, 3, 10]),
            vec![2, 3]
        );
    }
}
