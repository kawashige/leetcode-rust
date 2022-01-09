pub struct Solution {}

impl Solution {
    pub fn is_leap_year(year: i32) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }

    pub fn day_count(year: i32, month: i32) -> i32 {
        match month {
            2 if Self::is_leap_year(year) => 29,
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }

    pub fn day_of_year(date: String) -> i32 {
        let (year, month, day) = {
            let sp = date
                .split("-")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            (sp[0], sp[1], sp[2])
        };

        (1..month).map(|m| Self::day_count(year, m)).sum::<i32>() + day
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1154() {
        assert_eq!(Solution::day_of_year("2019-01-09".to_string()), 9);
        assert_eq!(Solution::day_of_year("2019-02-10".to_string()), 41);
    }
}
