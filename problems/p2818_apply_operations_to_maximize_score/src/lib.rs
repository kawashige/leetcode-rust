pub struct Solution {}

impl Solution {
    const M: usize = 1_000_000_007;

    pub fn pow(x: usize, n: usize) -> usize {
        let mut result = 1;
        let mut n = n;
        let mut x = x;
        while 0 < n {
            if n & 1 == 1 {
                result = (result * x) % Self::M;
            }
            x = (x * x) % Self::M;
            n >>= 1;
        }
        result
    }

    pub fn prime_score(n: i32) -> i32 {
        let mut remains = n as usize;
        let mut result = 0;
        for i in 2..=(n as f64).sqrt() as usize {
            if remains % i != 0 {
                continue;
            }
            result += 1;
            while remains % i == 0 {
                remains /= i;
            }
        }
        if remains != 1 {
            result += 1;
        }
        result
    }

    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let prime_score = nums
            .iter()
            .map(|n| Self::prime_score(*n))
            .collect::<Vec<_>>();

        let mut left = vec![-1; nums.len()];
        let mut stack = vec![0];
        for i in 1..nums.len() {
            while let Some(j) = stack.pop() {
                if prime_score[i] <= prime_score[j] {
                    stack.push(j);
                    break;
                }
            }
            if !stack.is_empty() {
                left[i] = *stack.last().unwrap() as i32;
            }
            stack.push(i);
        }

        let mut right = vec![nums.len() as i32; nums.len()];
        let mut stack = vec![nums.len() - 1];
        for i in (0..nums.len() - 1).rev() {
            while let Some(j) = stack.pop() {
                if prime_score[i] < prime_score[j] {
                    stack.push(j);
                    break;
                }
            }
            if !stack.is_empty() {
                right[i] = *stack.last().unwrap() as i32;
            }
            stack.push(i);
        }

        let mut result = 1;
        let mut sorted_nums = nums.iter().cloned().zip(0..).collect::<Vec<_>>();
        sorted_nums.sort_unstable_by_key(|v| -v.0);
        let mut k = k as usize;

        for i in 0..sorted_nums.len() {
            let d = ((sorted_nums[i].1 as i32 - left[sorted_nums[i].1]) as usize
                * (right[sorted_nums[i].1] - sorted_nums[i].1 as i32) as usize)
                .min(k as usize);
            result *= Self::pow(sorted_nums[i].0 as usize, d);
            result %= Self::M;
            k -= d;
            if k == 0 {
                break;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2818() {
        assert_eq!(
            Solution::maximum_score(vec![19, 12, 14, 6, 10, 18], 3),
            4788
        );
    }
}
