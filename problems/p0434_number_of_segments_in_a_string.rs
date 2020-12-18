pub struct Solution {}

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let s = s.trim();
        if s.is_empty() {
            return 0;
        }
        s.split(' ').filter(|ss| !ss.is_empty()).count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0434() {
        assert_eq!(
            5,
            Solution::count_segments("Hello, my name is John".to_string())
        );
        assert_eq!(1, Solution::count_segments("Hello".to_string()));
        assert_eq!(
            4,
            Solution::count_segments("love live! mu'sic forever".to_string())
        );
        assert_eq!(0, Solution::count_segments("".to_string()));
        assert_eq!(
            13,
            Solution::count_segments(
                "Of all the gin joints in all the towns in all the world,  ".to_string()
            )
        );
        assert_eq!(
            6,
            Solution::count_segments(", , , ,        a, eaefa".to_string())
        );
    }
}
