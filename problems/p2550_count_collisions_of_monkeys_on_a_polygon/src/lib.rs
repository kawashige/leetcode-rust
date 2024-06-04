pub struct Solution {}

impl Solution {
    const M: usize = 1_000_000_007;

    pub fn pow(n: usize) -> usize {
        if n == 1 {
            2
        } else if n % 2 == 0 {
            let x = Self::pow(n / 2);
            (x * x) % Self::M
        } else {
            (2 * Self::pow(n - 1)) % Self::M
        }
    }

    pub fn monkey_move(n: i32) -> i32 {
        let count = Self::pow(n as usize);
        ((count + Self::M - 2) % Self::M) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2550() {
        assert_eq!(Solution::monkey_move(3), 6);
        assert_eq!(Solution::monkey_move(4), 14);
    }
}
