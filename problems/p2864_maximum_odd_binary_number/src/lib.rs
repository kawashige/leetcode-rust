pub struct Solution {}

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let ones = s.as_bytes().iter().filter(|b| b == &&b'1').count();
        std::iter::repeat('1')
            .take(ones - 1)
            .chain(std::iter::repeat('0').take(s.len() - ones))
            .chain(std::iter::once('1'))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2864() {
        assert_eq!(
            Solution::maximum_odd_binary_number("010".to_string()),
            "001".to_string()
        );
        assert_eq!(
            Solution::maximum_odd_binary_number("0101".to_string()),
            "1001".to_string()
        );
    }
}
