pub struct Solution {}

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut prev1 = num.as_bytes()[0];
        let mut prev2 = num.as_bytes()[1];

        let mut found = [false; 10];

        for i in 2..num.len() {
            if prev1 == prev2 && prev2 == num.as_bytes()[i] {
                found[(num.as_bytes()[i] - b'0') as usize] = true;
            }
            prev1 = prev2;
            prev2 = num.as_bytes()[i];
        }

        (0..found.len())
            .rev()
            .find(|i| found[*i])
            .map(|i| format!("{}{}{}", i, i, i))
            .unwrap_or("".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2264() {
        assert_eq!(
            Solution::largest_good_integer("6777133339".to_string()),
            "777".to_string()
        );
        assert_eq!(
            Solution::largest_good_integer("2300019".to_string()),
            "000".to_string()
        );
        assert_eq!(
            Solution::largest_good_integer("42352338".to_string()),
            "".to_string()
        );
    }
}
