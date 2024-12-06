pub struct Solution {}

impl Solution {
    pub fn minimum_operations(num: String) -> i32 {
        let mut seen = vec![false; 10];
        for i in (0..num.len()).rev() {
            if (num.as_bytes()[i] == b'0' && seen[0])
                || (num.as_bytes()[i] == b'2' && seen[5])
                || (num.as_bytes()[i] == b'5' && seen[0])
                || (num.as_bytes()[i] == b'7' && seen[5])
            {
                return (num.len() - 1 - i - 1) as i32;
            }
            seen[(num.as_bytes()[i] - b'0') as usize] = true;
        }

        num.len() as i32 - if seen[0] { 1 } else { 0 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2844() {
        assert_eq!(Solution::minimum_operations("2245047".to_string()), 2);
        assert_eq!(Solution::minimum_operations("2908305".to_string()), 3);
        assert_eq!(Solution::minimum_operations("10".to_string()), 1);
    }
}
