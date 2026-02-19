use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn is_prime(val: i64) -> bool {
        if val == 1 {
            return false;
        }
        if val == 2 {
            return true;
        }
        for i in 2..=val {
            if val < i * i {
                break;
            }
            if val % i == 0 {
                return false;
            }
        }
        true
    }
    pub fn sum_of_largest_primes(s: String) -> i64 {
        let mut set = HashSet::new();

        for i in 0..s.len() {
            for j in 0..=i {
                let val = s[j..=i].parse::<i64>().unwrap();
                if Self::is_prime(val) {
                    set.insert(val);
                }
            }
        }

        let mut vals = set.into_iter().collect::<Vec<_>>();
        vals.sort_unstable_by(|a, b| b.cmp(&a));
        vals.into_iter().take(3).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3556() {
        assert_eq!(Solution::sum_of_largest_primes("12234".to_string()), 1469);
        assert_eq!(Solution::sum_of_largest_primes("111".to_string()), 11);
    }
}
