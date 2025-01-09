use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn last_visited_integers(nums: Vec<i32>) -> Vec<i32> {
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        let mut c = 0;
        for i in 0..nums.len() {
            if nums[i] == -1 {
                if c < queue.len() {
                    result.push(queue[c])
                } else {
                    result.push(-1);
                }
                c += 1;
            } else {
                c = 0;
                queue.push_front(nums[i]);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2899() {
        assert_eq!(
            Solution::last_visited_integers(vec![1, 2, -1, -1, -1]),
            vec![2, 1, -1]
        );
        assert_eq!(
            Solution::last_visited_integers(vec![1, -1, 2, -1, -1]),
            vec![1, 2, 1]
        );
    }
}
