pub struct Solution {}

use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};

#[derive(Eq, PartialEq, Clone, Debug)]
struct Pair {
    freq: i32,
    num: i32,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.freq != other.freq {
            self.freq.cmp(&other.freq)
        } else {
            self.num.cmp(&other.num)
        }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Helper {
    x: usize,
    result: i64,
    large: BTreeSet<Pair>,
    small: BTreeSet<Pair>,
    occ: HashMap<i32, i32>,
}

impl Helper {
    fn new(x: i32) -> Self {
        Helper {
            x: x as usize,
            result: 0,
            large: BTreeSet::new(),
            small: BTreeSet::new(),
            occ: HashMap::new(),
        }
    }

    fn insert(&mut self, num: i32) {
        if let Some(&count) = self.occ.get(&num) {
            if count > 0 {
                self.internal_remove(Pair { freq: count, num });
            }
        }
        *self.occ.entry(num).or_insert(0) += 1;
        let new_count = self.occ[&num];
        self.internal_insert(Pair {
            freq: new_count,
            num,
        });
    }

    fn remove(&mut self, num: i32) {
        let count = self.occ[&num];
        self.internal_remove(Pair { freq: count, num });
        *self.occ.get_mut(&num).unwrap() -= 1;
        if self.occ[&num] > 0 {
            let new_count = self.occ[&num];
            self.internal_insert(Pair {
                freq: new_count,
                num,
            });
        }
    }

    fn get(&self) -> i64 {
        self.result
    }

    fn internal_insert(&mut self, p: Pair) {
        if self.large.len() < self.x || p > *self.large.iter().next().unwrap() {
            self.result += p.freq as i64 * p.num as i64;
            self.large.insert(p.clone());
            if self.large.len() > self.x {
                let to_remove = self.large.iter().next().unwrap().clone();
                self.result -= to_remove.freq as i64 * to_remove.num as i64;
                self.large.remove(&to_remove);
                self.small.insert(to_remove);
            }
        } else {
            self.small.insert(p);
        }
    }

    fn internal_remove(&mut self, p: Pair) {
        if p >= *self.large.iter().next().unwrap() {
            self.result -= p.freq as i64 * p.num as i64;
            self.large.remove(&p);
            if let Some(to_add) = self.small.iter().next_back().cloned() {
                self.result += to_add.freq as i64 * to_add.num as i64;
                self.small.remove(&to_add);
                self.large.insert(to_add);
            }
        } else {
            self.small.remove(&p);
        }
    }
}

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        let mut helper = Helper::new(x);
        let mut ans = Vec::new();

        for i in 0..nums.len() {
            helper.insert(nums[i]);
            if i >= k as usize {
                helper.remove(nums[i - k as usize]);
            }
            if i >= (k - 1) as usize {
                ans.push(helper.get());
            }
        }

        ans
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3321() {
        assert_eq!(
            Solution::find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2),
            vec![6, 10, 12]
        );
        assert_eq!(
            Solution::find_x_sum(vec![3, 8, 7, 8, 7, 5], 2, 2),
            vec![11, 15, 15, 15, 12]
        );
    }
}
