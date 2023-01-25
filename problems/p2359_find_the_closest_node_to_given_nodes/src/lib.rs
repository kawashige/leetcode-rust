use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let mut dist = vec![vec![-1; edges.len()]; 2];
        let mut deque = VecDeque::new();
        deque.push_back((0, node1 as usize, 0));
        deque.push_back((1, node2 as usize, 0));

        while let Some((f, i, c)) = deque.pop_front() {
            dist[f][i] = c;

            if edges[i] != -1 && dist[f][edges[i] as usize] == -1 {
                deque.push_back((f, edges[i] as usize, c + 1));
            }
        }

        let mut max = std::i32::MAX;
        let mut max_i = 0;
        for i in 0..edges.len() {
            if dist[0][i] == -1 || dist[1][i] == -1 {
                continue;
            }
            if dist[0][i].max(dist[1][i]) < max {
                max = dist[0][i].max(dist[1][i]);
                max_i = i;
            }
        }

        if max == std::i32::MAX {
            -1
        } else {
            max_i as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2359() {
        assert_eq!(
            Solution::closest_meeting_node(vec![4, 4, 8, -1, 9, 8, 4, 4, 1, 1], 5, 6),
            1
        );
        assert_eq!(Solution::closest_meeting_node(vec![1, -1, 3, -1], 0, 2), -1);
        assert_eq!(Solution::closest_meeting_node(vec![2, 2, 3, -1], 0, 1), 2);
        assert_eq!(Solution::closest_meeting_node(vec![1, 2, -1], 0, 2), 2);
    }
}
