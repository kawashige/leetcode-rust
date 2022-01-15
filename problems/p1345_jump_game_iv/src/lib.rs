use std::collections::{HashMap, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut values = HashMap::new();
        for i in 0..arr.len() {
            (*values.entry(arr[i]).or_insert(Vec::new())).push(i);
        }

        let mut queue = VecDeque::new();
        queue.push_back((0, 0));

        let mut seen = vec![false; arr.len()];
        seen[0] = true;

        while let Some((i, c)) = queue.pop_front() {
            if i == arr.len() - 1 {
                return c;
            }

            if i > 0 && !seen[i - 1] {
                seen[i - 1] = true;
                queue.push_back((i - 1, c + 1));
            }
            if i + 1 < arr.len() && !seen[i + 1] {
                seen[i + 1] = true;
                queue.push_back((i + 1, c + 1));
            }

            if let Some(indices) = values.get(&arr[i]) {
                for j in indices {
                    if !seen[*j] {
                        seen[*j] = true;
                        queue.push_back((*j, c + 1));
                    }
                }
                values.remove(&arr[i]);
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1345() {
        assert_eq!(
            Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]),
            3
        );
        assert_eq!(Solution::min_jumps(vec![7]), 0);
        assert_eq!(Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]), 1);
    }
}
