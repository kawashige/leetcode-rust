pub struct Solution {}

impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        if n % 2 == 0 || n == 1 {
            n / 2
        } else {
            n
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2481() {
        assert_eq!(Solution::number_of_cuts(6), 3);
        assert_eq!(Solution::number_of_cuts(4), 2);
        assert_eq!(Solution::number_of_cuts(3), 3);
    }
}
