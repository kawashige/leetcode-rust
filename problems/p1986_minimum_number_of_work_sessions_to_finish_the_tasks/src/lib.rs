use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn recurse(
        remains: usize,
        tasks: &[i32],
        time: i32,
        session_time: i32,
        memo: &mut HashMap<(usize, i32), i32>,
    ) -> i32 {
        if remains == 0 {
            return time;
        }
        if let Some(result) = memo.get(&(remains, time)) {
            return *result;
        }

        let mut min_time = std::i32::MAX;

        for i in 0..tasks.len() {
            if remains & 1 << i == 0 {
                continue;
            }
            let new_time = time
                + tasks[i]
                + if session_time - time % session_time < tasks[i] {
                    session_time - time % session_time
                } else {
                    0
                };
            min_time = min_time.min(Self::recurse(
                remains ^ 1 << i,
                tasks,
                new_time,
                session_time,
                memo,
            ));
        }
        memo.insert((remains, time), min_time);
        min_time
    }

    pub fn min_sessions(tasks: Vec<i32>, session_time: i32) -> i32 {
        let time = Self::recurse(
            2_usize.pow(tasks.len() as u32) - 1,
            &tasks,
            0,
            session_time,
            &mut HashMap::new(),
        );
        (time + session_time - 1) / session_time
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1986() {
        assert_eq!(
            Solution::min_sessions(vec![1, 1, 2, 2, 2, 2, 3, 3, 6, 6, 6, 6], 10),
            4
        );
        assert_eq!(Solution::min_sessions(vec![7, 8], 9), 2);
        assert_eq!(Solution::min_sessions(vec![9, 6, 9], 14), 3);
        assert_eq!(Solution::min_sessions(vec![1, 2, 3], 3), 2);
        assert_eq!(Solution::min_sessions(vec![3, 1, 3, 1, 1], 8), 2);
        assert_eq!(Solution::min_sessions(vec![1, 2, 3, 4, 5], 15), 1);
    }
}
