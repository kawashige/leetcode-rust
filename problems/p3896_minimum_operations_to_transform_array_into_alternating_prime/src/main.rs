pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut is_prime = vec![true; 200_000];
        is_prime[1] = false;
        for i in 2..is_prime.len() {
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
        let prime_numbers = (0..is_prime.len())
            .filter(|i| is_prime[*i])
            .collect::<Vec<_>>();

        let mut nums = nums.into_iter().zip(0..).collect::<Vec<_>>();
        nums.sort_unstable();

        let mut result = 0;
        let mut j = 0;
        for i in 0..nums.len() {
            if nums[i].1 % 2 == 1 {
                if is_prime[nums[i].0 as usize] {
                    result += if nums[i].0 == 2 { 2 } else { 1 };
                }
            } else {
                if !is_prime[nums[i].0 as usize] {
                    while prime_numbers[j] < nums[i].0 as usize {
                        j += 1;
                    }
                    result += (prime_numbers[j] as i32 - nums[i].0).abs();
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3896() {
        assert_eq!(Solution::min_operations(vec![1, 2, 3, 4]), 3);
        assert_eq!(Solution::min_operations(vec![5, 6, 7, 8]), 0);
        assert_eq!(Solution::min_operations(vec![4, 4]), 1);
    }
}

fn main() {}
