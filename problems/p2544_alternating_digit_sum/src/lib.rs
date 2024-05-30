pub struct Solution {}

impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        n.to_string().chars().enumerate().fold(0, |acc, (i, c)| {
            acc + if i % 2 == 1 { -1 } else { 1 } * ((c as u8 - b'0') as i32)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2544() {
        assert_eq!(Solution::alternate_digit_sum(521), 4);
        assert_eq!(Solution::alternate_digit_sum(111), 1);
        assert_eq!(Solution::alternate_digit_sum(886996), 0);
    }
}
