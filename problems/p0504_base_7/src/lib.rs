pub struct Solution {}

impl Solution {
    pub fn convert_to_base7(mut num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }

        let mut result = Vec::new();
        if num < 0 {
            result.push("-".to_string());
            num = num.abs();
        }
        let n = (0..).find(|i| num < 7_i32.pow(*i)).unwrap();
        for i in (0..n).rev() {
            let p = 7_i32.pow(i);
            result.push((num / p).to_string());
            num %= p;
        }
        result.join("")
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0504() {
        assert_eq!("202".to_string(), Solution::convert_to_base7(100));
        assert_eq!("-10".to_string(), Solution::convert_to_base7(-7));
        assert_eq!("0".to_string(), Solution::convert_to_base7(0));
    }
}
