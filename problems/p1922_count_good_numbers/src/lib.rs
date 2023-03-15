pub struct Solution {}

impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        const M: i64 = 1_000_000_007;

        (match n {
            1 => 5,
            2 => 20,
            _ if n / 2 % 2 == 1 => 20 * Self::count_good_numbers(n - 2) as i64 % M,
            _ => {
                let x = Self::count_good_numbers(n / 2) as i64;
                if n % 2 == 0 {
                    x * x % M
                } else {
                    (x * x % M) * 5 % M
                }
            }
        }) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1922() {
        assert_eq!(Solution::count_good_numbers(1), 5);
        assert_eq!(Solution::count_good_numbers(4), 400);
        assert_eq!(Solution::count_good_numbers(50), 564908303);
    }
}
