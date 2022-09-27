use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        const M: usize = 1_000_000_007;

        let mut two_pow = vec![1; 22];
        for i in 1..two_pow.len() {
            two_pow[i] = 2 * two_pow[i - 1];
        }

        let counts = deliciousness.iter().fold(HashMap::new(), |mut counts, d| {
            *counts.entry(d).or_insert(0) += 1;
            counts
        });

        let mut count = 0;
        for d in counts.keys() {
            for p in &two_pow {
                if p < *d || &(p - *d) < *d {
                    continue;
                }
                if &(p - *d) == *d {
                    let c = counts.get(&(p - *d)).unwrap_or(&0);
                    if &1 < c {
                        count += c * (c - 1) / 2;
                    }
                } else {
                    let c1 = counts.get(*d).unwrap_or(&0);
                    let c2 = counts.get(&(p - *d)).unwrap_or(&0);
                    count += c1 * c2 % M;
                }
                count %= M;
            }
        }

        count as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1711() {
        assert_eq!(Solution::count_pairs(vec![1, 3, 5, 7, 9]), 4);
        assert_eq!(Solution::count_pairs(vec![1, 1, 1, 3, 3, 3, 7]), 15);
    }
}
