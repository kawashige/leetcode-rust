pub struct Solution {}

impl Solution {
    pub fn convert_to_title(n: i32) -> String {
        let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();
        let mut results = Vec::new();

        let mut remains = n;
        while remains > 0 {
            let i = (1..27).find(|num| (remains - num) % 26 == 0).unwrap();
            results.push(chars[i as usize - 1]);
            remains = (remains - i) / 26;
        }

        results.into_iter().rev().collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0168() {
        assert_eq!("AZ".to_string(), Solution::convert_to_title(52));
        assert_eq!("AZZ".to_string(), Solution::convert_to_title(1378));
        assert_eq!("AA".to_string(), Solution::convert_to_title(27));
        assert_eq!("A".to_string(), Solution::convert_to_title(1));
        assert_eq!("AB".to_string(), Solution::convert_to_title(28));
        assert_eq!("ZY".to_string(), Solution::convert_to_title(701));
    }
}
