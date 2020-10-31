pub struct Solution {}

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut primes = primes;
        primes.dedup();
        let mut next_nums: Vec<Vec<i32>> = primes.iter().cloned().map(|p| vec![0, p]).collect();

        let mut nums = vec![1];
        while nums.len() < n as usize {
            let j = (0..next_nums.len())
                .min_by_key(|i| next_nums[*i][1])
                .unwrap();
            nums.push(next_nums[j][1]);

            let mut next = next_nums[j][0] + 1;
            while next_nums
                .iter()
                .any(|n| n[1] == nums[next as usize] * primes[j])
            {
                next += 1;
            }
            next_nums[j] = vec![next, nums[next as usize] * primes[j]];
        }

        *nums.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0313() {
        assert_eq!(32, Solution::nth_super_ugly_number(12, vec![2, 7, 13, 19]));
        assert_eq!(10, Solution::nth_super_ugly_number(9, vec![2, 3, 5]));
    }
}
