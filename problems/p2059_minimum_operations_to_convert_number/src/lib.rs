use std::collections::{HashSet, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32 {
        let mut found = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));

        while let Some((x, c)) = queue.pop_front() {
            if x == goal {
                return c;
            }
            if found.contains(&x) || !(0..1001).contains(&x) {
                continue;
            }
            found.insert(x);

            for num in &nums {
                queue.push_back((x + num, c + 1));
                queue.push_back((x - num, c + 1));
                queue.push_back((x ^ num, c + 1));
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2059() {
        assert_eq!(Solution::minimum_operations(vec![2, 4, 12], 2, 12), 2);
        assert_eq!(Solution::minimum_operations(vec![3, 5, 7], 0, -4), 2);
        assert_eq!(Solution::minimum_operations(vec![2, 8, 16], 0, 1), -1);
    }
}
