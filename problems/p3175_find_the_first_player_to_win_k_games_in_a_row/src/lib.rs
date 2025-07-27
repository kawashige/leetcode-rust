use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
        if skills.len() - 1 <= k as usize {
            return (0..skills.len()).max_by_key(|i| skills[*i]).unwrap() as i32;
        }

        let mut queue = VecDeque::from_iter(0..skills.len());
        let mut wins = 0;
        while wins < k {
            let p1 = queue.pop_front().unwrap();
            if skills[p1] < skills[queue[0]] {
                queue.push_back(p1);
                wins = 1;
            } else {
                let p2 = queue.pop_front().unwrap();
                queue.push_back(p2);
                queue.push_front(p1);
                wins += 1;
            }
        }

        queue[0] as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3175() {
        assert_eq!(Solution::find_winning_player(vec![4, 2, 6, 3, 9], 2), 2);
        assert_eq!(Solution::find_winning_player(vec![2, 5, 4], 3), 1);
    }
}
