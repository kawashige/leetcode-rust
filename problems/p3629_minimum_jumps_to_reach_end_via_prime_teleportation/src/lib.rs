use std::collections::{HashMap, VecDeque};
use std::sync::OnceLock;

pub struct Solution {}
struct PrimeSieve {
    is_prime: Vec<bool>,
    prime_factors: Vec<Vec<usize>>,
}

static SIEVE: OnceLock<PrimeSieve> = OnceLock::new();
fn get_sieve() -> &'static PrimeSieve {
    SIEVE.get_or_init(|| {
        let max = 1_000_000;
        let mut is_prime = vec![true; max + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        let mut prime_factors = vec![vec![]; max + 1];
        for i in 2..prime_factors.len() {
            if !is_prime[i] {
                continue;
            }
            for j in (i + i..is_prime.len()).step_by(i) {
                is_prime[j] = false;
                prime_factors[j].push(i);
            }
        }
        PrimeSieve {
            is_prime,
            prime_factors,
        }
    })
}

impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        let sieve = get_sieve();
        let mut bucket = HashMap::new();
        for i in 0..nums.len() {
            if sieve.is_prime[nums[i] as usize] {
                (*bucket.entry(nums[i] as usize).or_insert(Vec::new())).push(i);
            } else {
                for j in &sieve.prime_factors[nums[i] as usize] {
                    (*bucket.entry(*j).or_insert(Vec::new())).push(i);
                }
            }
        }

        let mut queue = VecDeque::new();
        let mut seen = vec![false; nums.len()];
        queue.push_back((0, 0));

        while let Some((i, c)) = queue.pop_front() {
            if i == nums.len() - 1 {
                return c;
            }
            if seen[i] {
                continue;
            }
            seen[i] = true;

            if 0 < i && !seen[i - 1] {
                queue.push_back((i - 1, c + 1));
            }
            if i < nums.len() - 1 && !seen[i + 1] {
                queue.push_back((i + 1, c + 1));
            }
            if let Some(v) = bucket.get_mut(&(nums[i] as usize)) {
                while let Some(j) = v.pop() {
                    if !seen[j] {
                        queue.push_back((j, c + 1));
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
    fn test_3629() {
        assert_eq!(Solution::min_jumps(vec![5, 2, 20, 1, 15]), 1);
        assert_eq!(Solution::min_jumps(vec![7, 5, 7]), 1);
        assert_eq!(Solution::min_jumps(vec![1, 2, 4, 6]), 2);
        assert_eq!(Solution::min_jumps(vec![2, 3, 4, 7, 9]), 2);
        assert_eq!(Solution::min_jumps(vec![4, 6, 5, 8]), 3);
    }
}
