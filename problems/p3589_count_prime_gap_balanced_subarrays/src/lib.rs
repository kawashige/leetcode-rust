use std::collections::{BTreeMap, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn prime_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut is_prime = vec![true; 5 * 10_001];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 0..is_prime.len() {
            if is_prime.len() <= i * i {
                break;
            }
            if !is_prime[i] {
                continue;
            }
            for j in (i + i..is_prime.len()).step_by(i) {
                is_prime[j] = false;
            }
        }

        let mut primes = VecDeque::new();
        let mut map = BTreeMap::new();
        let mut l = 0;
        let mut result = 0;

        for r in 0..nums.len() {
            if is_prime[nums[r] as usize] {
                *map.entry(nums[r]).or_insert(0) += 1;
                primes.push_back(r);
            }
            while 0 < map.len() && k < map.keys().next_back().unwrap() - map.keys().next().unwrap()
            {
                if is_prime[nums[l] as usize] {
                    let freq = map.get_mut(&nums[l]).unwrap();
                    *freq -= 1;
                    if freq == &0 {
                        map.remove(&nums[l]);
                    }
                    primes.pop_front();
                }
                l += 1;
            }
            if 2 <= primes.len() {
                result += primes[primes.len() - 2] - l + 1;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3589() {
        assert_eq!(Solution::prime_subarray(vec![1, 3, 3], 1), 2);
        assert_eq!(Solution::prime_subarray(vec![2, 3, 5, 7], 3), 4);
    }
}
