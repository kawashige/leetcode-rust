use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn task_scheduler_ii(tasks: Vec<i32>, space: i32) -> i64 {
        let mut days = 0;
        let mut last_completed = HashMap::new();

        for i in 0..tasks.len() {
            if let Some(t) = last_completed.get(&tasks[i]) {
                if days < t + space as i64 {
                    days = t + space as i64
                }
            }
            days += 1;
            last_completed.insert(tasks[i], days);
        }

        days
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2365() {
        assert_eq!(Solution::task_scheduler_ii(vec![1, 2, 1, 2, 3, 1], 3), 9);
        assert_eq!(Solution::task_scheduler_ii(vec![5, 8, 8, 5], 2), 6);
    }
}
