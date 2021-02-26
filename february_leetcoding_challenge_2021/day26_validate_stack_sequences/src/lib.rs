pub struct Solution {}

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = Vec::with_capacity(pushed.len());
        let mut j = 0;
        for i in 0..pushed.len() {
            stack.push(pushed[i]);
            while stack.last().unwrap_or(&-1) == popped.get(j).unwrap_or(&1000) {
                stack.pop();
                j += 1;
            }
        }
        stack.is_empty() && j == popped.len()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day26() {
        assert!(!Solution::validate_stack_sequences(
            vec![],
            vec![4, 5, 3, 2, 1]
        ));
        assert!(Solution::validate_stack_sequences(
            vec![1, 2, 3, 4, 5],
            vec![4, 5, 3, 2, 1]
        ));
        assert!(!Solution::validate_stack_sequences(
            vec![1, 2, 3, 4, 5],
            vec![4, 3, 5, 1, 2]
        ));
    }
}
