use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        if arr.len() <= k as usize {
            return arr.into_iter().max().unwrap();
        }

        let mut arr = VecDeque::from(arr);
        let mut win_count = 0;
        let mut current = arr.pop_front().unwrap();

        while win_count < k {
            let next = arr.pop_front().unwrap();
            if current < next {
                arr.push_back(current);
                current = next;
                win_count = 1;
            } else {
                arr.push_back(next);
                win_count += 1;
            }
        }

        current
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1535() {
        assert_eq!(Solution::get_winner(vec![2, 1, 3, 5, 4, 6, 7], 2), 5);
        assert_eq!(Solution::get_winner(vec![3, 1, 2], 3), 3);
    }
}
