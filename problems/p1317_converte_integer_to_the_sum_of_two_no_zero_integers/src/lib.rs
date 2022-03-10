pub struct Solution {}

impl Solution {
    pub fn is_no_zero(n: i32) -> bool {
        !n.to_string().contains('0')
    }

    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        for i in 1..n {
            if Self::is_no_zero(i) && Self::is_no_zero(n - i) {
                return vec![i, n - i];
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1317() {
        assert_eq!(Solution::get_no_zero_integers(2), vec![1, 1]);
        assert_eq!(Solution::get_no_zero_integers(11), vec![2, 9]);
    }
}
