pub struct Solution {}

impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let mut result: Vec<i32> = (0..2_i32.pow(n as u32)).map(|x| x ^ (x >> 1)).collect();
        let i = (0..result.len()).find(|i| result[*i] == start).unwrap();
        result.rotate_left(i);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1238() {
        assert_eq!(Solution::circular_permutation(2, 3), vec![3, 2, 0, 1]);
        assert_eq!(
            Solution::circular_permutation(3, 2),
            vec![2, 6, 7, 5, 4, 0, 1, 3]
        );
    }
}
