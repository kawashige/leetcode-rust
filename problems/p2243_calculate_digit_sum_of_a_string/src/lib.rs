pub struct Solution {}

impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let mut s = s;
        let k = k as usize;

        while k < s.len() {
            let mut new_s = String::new();
            let mut sum = 0;
            for i in 0..s.len() {
                sum += (s.as_bytes()[i] - b'0') as usize;
                if (i + 1) % k == 0 || i == s.len() - 1 {
                    new_s += &sum.to_string();
                    sum = 0;
                }
            }
            s = new_s;
        }

        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2243() {
        assert_eq!(
            Solution::digit_sum("11111222223".to_string(), 3),
            "135".to_string()
        );
        assert_eq!(
            Solution::digit_sum("00000000".to_string(), 3),
            "000".to_string()
        );
    }
}
