pub struct Solution {}

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let c_num1 = num1.chars().collect::<Vec<char>>();
        let c_num2 = num2.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut j = 0;
        let mut c = 0;
        let mut results = Vec::new();
        while i < c_num1.len() || j < c_num2.len() {
            let n1 = if i < c_num1.len() {
                c_num1[c_num1.len() - 1 - i].to_digit(10).unwrap()
            } else {
                0
            };
            let n2 = if i < c_num2.len() {
                c_num2[c_num2.len() - 1 - j].to_digit(10).unwrap()
            } else {
                0
            };
            let s = c + n1 + n2;
            c = if 9 < s { 1 } else { 0 };
            results.push(s.to_string().chars().last().unwrap());
            i += 1;
            j += 1;
        }
        if c == 1 {
            results.push('1');
        }
        results.into_iter().rev().collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0415() {
        assert_eq!(
            "21".to_string(),
            Solution::add_strings("12".to_string(), "9".to_string())
        );
        assert_eq!(
            "100".to_string(),
            Solution::add_strings("91".to_string(), "9".to_string())
        );
        assert_eq!(
            "0".to_string(),
            Solution::add_strings("0".to_string(), "0".to_string())
        );
    }
}
