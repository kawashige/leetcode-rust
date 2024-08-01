pub struct Solution {}

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .into_iter()
            .filter(|d| 60 < d[11..13].parse::<usize>().unwrap())
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2678() {
        assert_eq!(
            Solution::count_seniors(vec![
                "7868190130M7522".to_string(),
                "5303914400F9211".to_string(),
                "9273338290F4010".to_string()
            ]),
            2
        );
        assert_eq!(
            Solution::count_seniors(vec![
                "1313579440F2036".to_string(),
                "2921522980M5644".to_string()
            ]),
            0
        );
    }
}
