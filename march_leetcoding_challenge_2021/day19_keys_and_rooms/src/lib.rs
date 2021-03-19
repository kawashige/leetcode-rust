use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut seen = vec![false; rooms.len()];
        let mut queue = VecDeque::new();
        queue.push_back(0);

        while let Some(r) = queue.pop_front() {
            if seen[r] {
                continue;
            }

            seen[r] = true;
            for n in &rooms[r] {
                queue.push_back(*n as usize);
            }
        }

        seen.iter().all(|t| *t)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day19() {
        assert!(Solution::can_visit_all_rooms(vec![
            vec![1],
            vec![2],
            vec![3],
            vec![]
        ]));
        assert!(!Solution::can_visit_all_rooms(vec![
            vec![1, 3],
            vec![3, 0, 1],
            vec![2],
            vec![0]
        ]));
    }
}
