pub struct Solution {}

impl Solution {
    pub fn smallest_unique_subarray(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        const BASE: i64 = 911_382_323;

        let n = nums.len();
        let mut hash = vec![0_i64; n + 1];
        let mut powers = vec![0_i64; n + 1];
        powers[0] = 1;

        // Compute prefix hashes and powers of BASE
        for (i, &num) in nums.iter().enumerate() {
            hash[i + 1] = (hash[i] * BASE + i64::from(num)) % MOD;
            powers[i + 1] = powers[i] * BASE % MOD;
        }

        let check = |length: usize| -> bool {
            let mut counts = std::collections::HashMap::with_capacity(n - length + 1);

            for left in 0..=n - length {
                let right = left + length;

                let key = (hash[right] - hash[left] * powers[length] % MOD + MOD) % MOD;
                *counts.entry(key).or_insert(0_usize) += 1;
            }

            counts.values().any(|&times| times == 1)
        };

        let (mut lo, mut hi) = (1, n);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            if check(mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        lo as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3934() {
        assert_eq!(Solution::smallest_unique_subarray(vec![3, 3, 3]), 3);
        assert_eq!(Solution::smallest_unique_subarray(vec![2, 1, 2, 3, 3]), 1);
        assert_eq!(Solution::smallest_unique_subarray(vec![1, 1, 2, 2, 1]), 2);
    }
}

fn main() {}
