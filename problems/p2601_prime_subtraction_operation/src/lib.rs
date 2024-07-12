pub struct Solution {}

impl Solution {
    pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
        let mut is_prime = vec![true; *nums.iter().max().unwrap() as usize];
        let mut primes = Vec::new();
        for i in 2..is_prime.len() {
            if !is_prime[i] {
                continue;
            }
            primes.push(i as i32);
            for j in ((i + i)..is_prime.len()).step_by(i) {
                is_prime[j] = false;
            }
        }

        let mut nums = nums;
        for i in 0..nums.len() {
            let prev = if i == 0 { 0 } else { nums[i - 1] };
            if let Some(j) = (0..primes.len())
                .rev()
                .find(|j| primes[*j] < nums[i] && prev < nums[i] - primes[*j])
            {
                nums[i] -= primes[j];
            }
            if 0 < i && nums[i - 1] >= nums[i] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2601() {
        assert!(Solution::prime_sub_operation(vec![4, 9, 6, 10]));
        assert!(Solution::prime_sub_operation(vec![6, 8, 11, 12]));
        assert!(!Solution::prime_sub_operation(vec![5, 8, 3]));
    }
}
