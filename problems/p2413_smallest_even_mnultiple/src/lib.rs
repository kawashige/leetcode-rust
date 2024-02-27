pub struct Solution {}

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        if n % 2 == 0 {
            n
        } else {
            2 * n
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2413() {
        assert_eq!(Solution::smallest_even_multiple(5), 10);
        assert_eq!(Solution::smallest_even_multiple(6), 6);
    }
}
