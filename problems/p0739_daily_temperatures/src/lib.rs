pub struct Solution {}

impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; t.len()];
        let mut stack = Vec::new();
        for i in (0..t.len()).rev() {
            while !stack.is_empty() && t[i] >= t[*stack.last().unwrap()] {
                stack.pop();
            }
            result[i] = if stack.is_empty() {
                0
            } else {
                (stack.last().unwrap() - i) as i32
            };
            stack.push(i);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0739() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![89, 62, 70, 58, 47, 47, 46, 76, 100, 70]),
            vec![8, 1, 5, 4, 3, 2, 1, 1, 0, 0]
        )
    }
}
