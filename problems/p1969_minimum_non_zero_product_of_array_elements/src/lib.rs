pub struct Solution {}

impl Solution {
    const M: usize = 1_000_000_007;

    pub fn pow(x: usize, n: usize) -> usize {
        if n == 0 {
            1
        } else if n % 2 == 0 {
            let p = Self::pow(x, n / 2);
            p * p % Self::M
        } else {
            x * Self::pow(x, n - 1) % Self::M
        }
    }

    pub fn min_non_zero_product(p: i32) -> i32 {
        let num = (1..p).fold(0, |acc, i| acc | 1 << i);
        (((num | 1) % Self::M) * Self::pow(num % Self::M, 2_usize.pow(p as u32 - 1) - 1) % Self::M)
            as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1969() {
        assert_eq!(Solution::min_non_zero_product(1), 1);
        assert_eq!(Solution::min_non_zero_product(2), 6);
        assert_eq!(Solution::min_non_zero_product(3), 1512);
    }
}
