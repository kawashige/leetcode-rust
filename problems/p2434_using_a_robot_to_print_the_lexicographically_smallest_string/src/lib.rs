pub struct Solution {}

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut smallest = vec![s.as_bytes()[s.len() - 1]; s.len()];
        for i in (0..s.len() - 1).rev() {
            smallest[i] = smallest[i + 1].min(s.as_bytes()[i] as u8);
        }

        let mut result = String::new();
        let mut t = Vec::with_capacity(s.len());

        for i in 0..s.len() {
            while !t.is_empty() && t.last().unwrap() <= &smallest[i] {
                result.push(t.pop().unwrap() as char);
            }
            t.push(s.as_bytes()[i]);
        }
        while !t.is_empty() {
            result.push(t.pop().unwrap() as char);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2434() {
        assert_eq!(
            Solution::robot_with_string("zza".to_string()),
            "azz".to_string()
        );
        assert_eq!(
            Solution::robot_with_string("bac".to_string()),
            "abc".to_string()
        );
        assert_eq!(
            Solution::robot_with_string("bdda".to_string()),
            "addb".to_string()
        );
    }
}
