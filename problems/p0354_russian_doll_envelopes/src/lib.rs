pub struct Solution {}

use std::collections::{BTreeMap, BTreeSet};

struct Bit {
    n: usize,
    pub data: Vec<usize>,
}

impl Bit {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![0; n + 1],
        }
    }

    pub fn max(&self, i: usize) -> usize {
        let mut r = 0;
        let mut i = i as i32;
        while i > 0 {
            r = std::cmp::max(r, self.data[i as usize]);
            i -= i & -i;
        }
        r
    }

    pub fn add(&mut self, i: usize, x: usize) {
        let mut i = i as i32;
        while i as usize <= self.n {
            self.data[i as usize] = std::cmp::max(x, self.data[i as usize]);
            i += i & -i;
        }
    }
}

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut map = BTreeMap::new();
        for e in envelopes {
            (*map.entry(e[1]).or_insert(BTreeSet::new())).insert(e[0]);
        }

        let mut bit = Bit::new(100_000);
        for (_, widths) in map {
            for w in widths.into_iter().rev() {
                bit.add(w as usize, bit.max(w as usize - 1) + 1);
            }
        }

        bit.max(100_000) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0354() {
        assert_eq!(
            Solution::max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]),
            3
        );
        assert_eq!(
            Solution::max_envelopes(vec![vec![1, 1], vec![1, 1], vec![1, 1]]),
            1
        );
    }
}
