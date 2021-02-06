pub struct Solution {}

use std::collections::{BinaryHeap, VecDeque};
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut remains = [0; 26];
        for task in tasks {
            remains[task as usize - 0x41] += 1;
        }

        let mut heap = BinaryHeap::new();
        for i in 0..26 {
            if 0 < remains[i] {
                heap.push(remains[i]);
            }
        }

        let mut timing = 0;
        let mut cooldown: VecDeque<(i32, i32)> = VecDeque::new();

        while !heap.is_empty() || !cooldown.is_empty() {
            if !cooldown.is_empty() && n < timing - cooldown[0].0 {
                heap.push(cooldown.pop_front().unwrap().1);
            }
            if let Some(remains) = heap.pop() {
                if 1 < remains {
                    if n == 0 {
                        heap.push(remains - 1);
                    } else {
                        cooldown.push_back((timing, remains - 1));
                    }
                }
            }
            timing += 1;
        }

        timing
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0621() {
        assert_eq!(
            Solution::least_interval(
                vec!['A', 'A', 'A', 'B', 'B', 'B', 'C', 'C', 'C', 'D', 'D', 'E'],
                2
            ),
            12
        );
        assert_eq!(Solution::least_interval(vec!['A'], 100), 1);
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2),
            8
        );
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0),
            6
        );
        assert_eq!(
            Solution::least_interval(
                vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
                2
            ),
            16
        );
    }
}
