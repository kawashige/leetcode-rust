pub struct Solution {}

impl Solution {
    const M: u64 = 1_000_000_007;

    pub fn modinv(a: u64) -> u64 {
        let mut a = a as i64;
        let m = 1_000_000_007;
        let mut b = m;
        let mut u = 1;
        let mut v = 0;
        while b > 0 {
            let t = a / b;
            a -= t * b;
            std::mem::swap(&mut a, &mut b);
            u -= t * v;
            std::mem::swap(&mut u, &mut v);
        }
        u %= m;
        if u < 0 {
            u += m;
        }
        u as u64
    }

    pub fn recurse(nums: &[i32], factorial: &[u64]) -> u64 {
        if nums.len() <= 1 {
            return 1;
        }

        let mut smaller = Vec::new();
        let mut bigger = Vec::new();

        for i in 1..nums.len() {
            if nums[i] < nums[0] {
                smaller.push(nums[i]);
            } else {
                bigger.push(nums[i]);
            }
        }

        let smaller_count = Self::recurse(&smaller, factorial);
        let bigger_count = Self::recurse(&bigger, factorial);
        let count = (factorial[nums.len() - 1] * Self::modinv(factorial[smaller.len()]) % Self::M)
            * Self::modinv(factorial[bigger.len()])
            % Self::M;

        ((smaller_count * bigger_count) % Self::M) * count % Self::M
    }

    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let mut factorial = vec![1; nums.len() + 1];
        for i in 1..factorial.len() {
            factorial[i] = factorial[i - 1] * i as u64 % Self::M;
        }

        ((Self::recurse(&nums, &factorial) + Self::M - 1) % Self::M) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1569() {
        assert_eq!(Solution::num_of_ways(vec![2, 1, 3]), 1);
        assert_eq!(Solution::num_of_ways(vec![3, 4, 5, 1, 2]), 5);
        assert_eq!(Solution::num_of_ways(vec![1, 2, 3]), 0);
    }
}
