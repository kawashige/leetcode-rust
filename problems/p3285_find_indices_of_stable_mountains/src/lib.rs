pub struct Solution {}

impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        (1..height.len() as i32)
            .filter(|i| threshold < height[*i as usize - 1])
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3285() {
        assert_eq!(
            Solution::stable_mountains(vec![1, 2, 3, 4, 5], 2),
            vec![3, 4]
        );
        assert_eq!(
            Solution::stable_mountains(vec![10, 1, 10, 1, 10], 3),
            vec![1, 3]
        );
        assert_eq!(
            Solution::stable_mountains(vec![10, 1, 10, 1, 10], 10),
            vec![]
        );
    }
}
