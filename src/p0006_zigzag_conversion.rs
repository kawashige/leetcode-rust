pub struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut results = vec![Vec::new(); num_rows as usize];
        let mut i: i32 = 0;
        let mut add: i32 = -1;
        for c in s.chars() {
            results[i as usize].push(c);
            if num_rows == 1 {
                continue;
            }
            if i == 0 || i == num_rows - 1 {
                add *= -1;
            }
            i += add;
        }

        results
            .iter()
            .map(|chars| chars.iter().collect::<String>())
            .collect::<Vec<String>>()
            .concat()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0006() {
        assert_eq!(
            "PAHNAPLSIIGYIR".to_string(),
            Solution::convert("PAYPALISHIRING".to_string(), 3)
        );
        assert_eq!("AB".to_string(), Solution::convert("AB".to_string(), 1));
    }
}
