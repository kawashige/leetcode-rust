pub struct Solution {}

impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        let bulky = [length, width, height].iter().any(|l| &10_000 <= l)
            || 1_000_000_000 <= (length as i64 * width as i64 * height as i64);
        let heavy = 100 <= mass;

        match (bulky, heavy) {
            (true, true) => "Both".to_string(),
            (true, false) => "Bulky".to_string(),
            (false, true) => "Heavy".to_string(),
            _ => "Neither".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2525() {
        assert_eq!(
            Solution::categorize_box(1000, 35, 700, 300),
            "Heavy".to_string()
        );
        assert_eq!(
            Solution::categorize_box(200, 50, 800, 50),
            "Neither".to_string()
        );
    }
}
