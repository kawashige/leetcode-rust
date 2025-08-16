use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        let mut power = power;
        power.sort_unstable();

        let mut queue: VecDeque<(i32, i64)> = VecDeque::new();
        for p in power {
            let l = if queue.is_empty() { 0 } else { queue.len() - 1 };
            if !queue.is_empty() && queue[l].0 == p {
                queue[l].1 += p as i64;
            } else {
                let mut max = 0;
                for i in 0..queue.len() {
                    if 2 < p - queue[i].0 {
                        max = max.max(queue[i].1);
                    }
                }
                queue.push_back((p, max + p as i64));
                if queue.len() == 6 {
                    queue.pop_front();
                }
            }
        }

        queue.into_iter().map(|q| q.1).max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3186() {
        assert_eq!(Solution::maximum_total_damage(vec![1, 1, 3, 4]), 6);
        assert_eq!(Solution::maximum_total_damage(vec![7, 1, 6, 6]), 13);
    }
}
