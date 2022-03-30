pub struct Solution {}

impl Solution {
    pub fn is_leap_year(year: i32) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }

    pub fn month_day_count(year: i32, month: i32) -> i32 {
        match month {
            2 if Self::is_leap_year(year) => 29,
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }

    pub fn year_day_count(year: i32) -> i32 {
        365 + if Self::is_leap_year(year) { 1 } else { 0 }
    }

    pub fn days_of_date(date: String) -> i32 {
        let (year, month, day) = {
            let sp = date
                .split("-")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            (sp[0], sp[1], sp[2])
        };

        (1971..year).map(|y| Self::year_day_count(y)).sum::<i32>()
            + (1..month)
                .map(|m| Self::month_day_count(year, m))
                .sum::<i32>()
            + day
    }

    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        (Self::days_of_date(date1) - Self::days_of_date(date2)).abs()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1360() {
        assert_eq!(
            Solution::days_between_dates("2019-06-29".to_string(), "2019-06-30".to_string()),
            1
        );
        assert_eq!(
            Solution::days_between_dates("2020-01-15".to_string(), "2019-12-31".to_string()),
            15
        );
    }
}
