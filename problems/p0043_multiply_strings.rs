pub struct Solution {}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        let mut chars1 = num1.chars().collect::<Vec<char>>();
        let mut chars2 = num2.chars().collect::<Vec<char>>();
        chars1.reverse();
        chars2.reverse();

        let mut results = Vec::new();
        let mut carry = 0;
        for i in 0..(chars1.len() + chars2.len() - 1) {
            let mut sum = carry;
            for j in 0..chars2.len() {
                if 0 <= i as i32 - j as i32 && i - j < chars1.len() {
                    sum += chars1[i - j].to_digit(10).unwrap() * chars2[j].to_digit(10).unwrap();
                }
            }
            let sum_chars = sum.to_string().chars().collect::<Vec<char>>();
            if sum_chars.len() == 1 {
                results.push(sum_chars[0]);
                carry = 0;
            } else {
                results.push(sum_chars[sum_chars.len() - 1]);
                carry = sum_chars[..(sum_chars.len() - 1)]
                    .iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
            }
        }
        if carry != 0 {
            results.push(std::char::from_digit(carry, 10).unwrap());
        }
        results.reverse();
        results.iter().collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0043() {
        assert_eq!(
            "6".to_string(),
            Solution::multiply("2".to_string(), "3".to_string())
        );
        assert_eq!(
            "56088".to_string(),
            Solution::multiply("123".to_string(), "456".to_string())
        );
        assert_eq!(
            "50".to_string(),
            Solution::multiply("10".to_string(), "5".to_string())
        );
        assert_eq!(
            "0".to_string(),
            Solution::multiply("0".to_string(), "52222".to_string())
        );
        assert_eq!(
            "998001".to_string(),
            Solution::multiply("999".to_string(), "999".to_string())
        );
    }
}
