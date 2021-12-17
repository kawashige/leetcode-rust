use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut queue = VecDeque::new();
        for i in 0..arr.len() {
            let target = if let Some(x) = queue.pop_front() {
                queue.push_back(arr[i]);
                x
            } else {
                arr[i]
            };
            if arr[i] == 0 {
                queue.push_back(arr[i]);
            }

            arr[i] = target;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1089() {
        let mut r = vec![1, 0, 2, 3, 0, 4, 5, 0];
        Solution::duplicate_zeros(&mut r);
        assert_eq!(r, vec![1, 0, 0, 2, 3, 0, 0, 4]);

        let mut r = vec![1, 2, 3];
        Solution::duplicate_zeros(&mut r);
        assert_eq!(r, vec![1, 2, 3]);
        let mut r = vec![1, 0, 0, 3, 0, 4, 5, 0];
        Solution::duplicate_zeros(&mut r);
        assert_eq!(r, vec![1, 0, 0, 0, 0, 3, 0, 0]);
    }
}
