pub struct Solution {}

impl Solution {
    pub fn check_prime_frequency(nums: Vec<i32>) -> bool {
        let mut is_prime = vec![true; 101];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..is_prime.len() {
            if is_prime.len() <= i * i {
                continue;
            }
            if !is_prime[i] {
                continue;
            }
            for j in (i + i..is_prime.len()).step_by(i) {
                is_prime[j] = false;
            }
        }
        nums.into_iter()
            .fold([0; 101], |mut count, x| {
                count[x as usize] += 1;
                count
            })
            .into_iter()
            .any(|c| is_prime[c])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3591() {
        assert!(Solution::check_prime_frequency(vec![1, 2, 3, 4, 5, 4]));
        assert!(!Solution::check_prime_frequency(vec![1, 2, 3, 4, 5]));
        assert!(Solution::check_prime_frequency(vec![2, 2, 2, 4, 4]));
    }
}
