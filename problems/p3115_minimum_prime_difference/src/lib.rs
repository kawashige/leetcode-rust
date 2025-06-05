pub struct Solution {}

impl Solution {
    pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        let mut is_prime = vec![true; 101];
        is_prime[1] = false;
        for i in 2..is_prime.len() {
            if !is_prime[i] {
                continue;
            }
            for j in (i + i..is_prime.len()).step_by(i) {
                is_prime[j] = false;
            }
        }

        ((0..nums.len())
            .rev()
            .find(|i| is_prime[nums[*i] as usize])
            .unwrap()
            - (0..nums.len())
                .find(|i| is_prime[nums[*i] as usize])
                .unwrap()) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3115() {
        assert_eq!(Solution::maximum_prime_difference(vec![4, 2, 9, 5, 3]), 3);
        assert_eq!(Solution::maximum_prime_difference(vec![4, 8, 2, 8]), 0);
    }
}
