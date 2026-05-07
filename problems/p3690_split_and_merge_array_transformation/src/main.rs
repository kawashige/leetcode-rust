use std::collections::{HashSet, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn min_split_merge(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let nums1 = nums1.into_iter().map(|n| n.to_string()).collect::<Vec<_>>();
        let nums2 = nums2.into_iter().map(|n| n.to_string()).collect::<Vec<_>>();
        let mut seen = HashSet::new();
        let mut deque = VecDeque::new();
        deque.push_back((nums1, 0));

        while let Some((state, count)) = deque.pop_front() {
            if (0..state.len()).all(|i| state[i] == nums2[i]) {
                return count;
            }
            let key = state.join(",");
            if seen.contains(&key) {
                continue;
            }
            seen.insert(key);
            for l in 0..state.len() {
                for r in l..state.len() {
                    let mut removed_state = state[..l].to_vec();
                    removed_state.extend_from_slice(&state[r + 1..]);
                    for i in 0..removed_state.len() {
                        let mut new_state = removed_state.clone();
                        for j in 0..r - l + 1 {
                            new_state.insert(i + j, state[l + j].clone());
                        }
                        deque.push_back((new_state, count + 1));
                    }
                }
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3690() {
        assert_eq!(Solution::min_split_merge(vec![3, 1, 2], vec![1, 2, 3]), 1);
        assert_eq!(
            Solution::min_split_merge(vec![1, 1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1, 1]),
            3
        );
    }
}

fn main() {}
