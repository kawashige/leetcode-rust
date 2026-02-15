use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn max_weight(n: i32, edges: Vec<Vec<i32>>, k: i32, t: i32) -> i32 {
        if k == 0 {
            return 0;
        }
        let n = n as usize;
        let mut list = vec![vec![]; n];
        let mut stack = Vec::new();
        for e in edges {
            list[e[0] as usize].push((e[1] as usize, e[2]));
            if e[2] < t {
                stack.push((e[1] as usize, 1, e[2]));
            }
        }

        let mut result = -1;
        let mut seen = HashSet::new();
        while let Some((i, c, w)) = stack.pop() {
            if seen.contains(&(i, c, w)) {
                continue;
            }
            seen.insert((i, c, w));
            if c == k as usize {
                result = result.max(w);
                continue;
            }

            for (next, next_w) in &list[i] {
                if w + next_w < t {
                    stack.push((*next, c + 1, w + next_w));
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3543() {
        assert_eq!(
            Solution::max_weight(3, vec![vec![0, 1, 1], vec![1, 2, 2]], 2, 4),
            3
        );
        assert_eq!(
            Solution::max_weight(3, vec![vec![0, 1, 2], vec![0, 2, 3]], 1, 3),
            2
        );
        assert_eq!(
            Solution::max_weight(3, vec![vec![0, 1, 6], vec![1, 2, 8]], 1, 6),
            -1
        );
    }
}
