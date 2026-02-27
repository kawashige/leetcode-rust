pub struct Solution {}

use std::cmp;
use std::collections::{BTreeSet, VecDeque};

impl Solution {
    pub fn min_operations(s: String, k: i32) -> i32 {
        let k = k as usize;
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut m = 0;
        let mut dist = vec![i32::MAX; n + 1];
        let mut node_sets = [BTreeSet::new(), BTreeSet::new()];

        for i in 0..=n {
            node_sets[i % 2].insert(i);
            if i < n && chars[i] == '0' {
                m += 1;
            }
        }

        let mut q = VecDeque::new();
        q.push_back(m);
        dist[m] = 0;
        node_sets[m % 2].remove(&m);

        while let Some(current_m) = q.pop_front() {
            let c1 = cmp::max(k as i32 - n as i32 + current_m as i32, 0) as usize;
            let c2 = cmp::min(current_m, k);

            let lnode = current_m + k - 2 * c2;
            let rnode = current_m + k - 2 * c1;
            let node_set = &mut node_sets[lnode % 2];
            let range: Vec<usize> = node_set.range(lnode..=rnode).copied().collect();

            for m2 in range {
                dist[m2] = dist[current_m] + 1;
                q.push_back(m2);
                node_set.remove(&m2);
            }
        }

        if dist[0] == i32::MAX {
            -1
        } else {
            dist[0]
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3666() {
        assert_eq!(Solution::min_operations("110".to_string(), 1), 1);
        assert_eq!(Solution::min_operations("0101".to_string(), 3), 2);
        assert_eq!(Solution::min_operations("101".to_string(), 2), -1);
    }
}
