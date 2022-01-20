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
        (0..12)
            .map(|month| Self::month_day_count(year, month))
            .sum()
    }

    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        const DOW: [&str; 7] = [
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
        ];

        let week: i32 = (5
            + (1971..year).map(|y| Self::year_day_count(y)).sum::<i32>()
            + (1..month)
                .map(|m| Self::month_day_count(year, m))
                .sum::<i32>()
            + day
            - 1)
            % 7;

        DOW[week as usize].into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1185() {
        assert_eq!(
            Solution::day_of_the_week(31, 8, 2019),
            "Saturday".to_string()
        );
        assert_eq!(Solution::day_of_the_week(18, 7, 1999), "Sunday".to_string());
        assert_eq!(Solution::day_of_the_week(15, 8, 1993), "Sunday".to_string());
    }
}
