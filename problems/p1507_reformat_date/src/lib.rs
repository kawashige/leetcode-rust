pub struct Solution {}

impl Solution {
    pub fn reformat_date(date: String) -> String {
        let months = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];
        let (y, m, d) = {
            let mut v = date.split_ascii_whitespace();
            let d = v
                .next()
                .unwrap()
                .trim_end_matches(char::is_alphabetic)
                .parse::<usize>()
                .unwrap();
            let m = v.next().unwrap();
            let m = (0..12).find(|i| months[*i] == m).unwrap() + 1;
            let y = v.next().unwrap();

            (y, m, d)
        };

        format!("{}-{:02}-{:02}", y, m, d)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1507() {
        assert_eq!(
            Solution::reformat_date("20th Oct 2052".to_string()),
            "2052-10-20".to_string()
        );
        assert_eq!(
            Solution::reformat_date("6th Jun 1933".to_string()),
            "1933-06-06".to_string()
        );
        assert_eq!(
            Solution::reformat_date("26th May 1960".to_string()),
            "1960-05-26".to_string()
        );
    }
}
