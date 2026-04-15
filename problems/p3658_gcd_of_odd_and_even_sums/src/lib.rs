pub struct Solution {}

impl Solution {
    pub fn gcd(m: i32, n: i32) -> i32 {
        if m == 0 {
            return n;
        } else {
            Self::gcd(n % m, m)
        }
    }

    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        let odd_sum = n * (2 + (n - 1) * 2) / 2;
        let even_sum = n * (4 + (n - 1) * 2) / 2;
        Self::gcd(odd_sum, even_sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3658() {
        assert_eq!(Solution::gcd_of_odd_even_sums(4), 4);
        assert_eq!(Solution::gcd_of_odd_even_sums(5), 5);
    }
}
