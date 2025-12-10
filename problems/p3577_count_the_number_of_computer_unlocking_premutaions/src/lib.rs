pub struct Solution {}

impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        if complexity[1..].iter().any(|v| v <= &complexity[0]) {
            return 0;
        }
        const M: i64 = 1_000_000_007;
        (1..complexity.len() as i64).fold(1, |acc, x| (acc * x) % M) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3577() {
        assert_eq!(Solution::count_permutations(vec![1, 2, 3]), 2);
        assert_eq!(Solution::count_permutations(vec![3, 3, 3, 4, 4, 4]), 0);
    }
}
