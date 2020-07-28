pub struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a = i128::from_str_radix(&a, 2).unwrap();
        let b = i128::from_str_radix(&b, 2).unwrap();
        format!("{:b}", a + b)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_67() {
        let ret = Solution::add_binary("1010".to_string(), "1011".to_string());
        assert_eq!("10101", ret);
    }
}
