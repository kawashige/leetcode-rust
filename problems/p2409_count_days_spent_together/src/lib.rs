pub struct Solution {}

impl Solution {
    pub fn convert_to_index(date: String) -> i32 {
        let days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let sp = date
            .split('-')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        days[..sp[0] - 1].iter().sum::<i32>() + sp[1] as i32
    }

    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let alice = (
            Self::convert_to_index(arrive_alice),
            Self::convert_to_index(leave_alice),
        );
        let bob = (
            Self::convert_to_index(arrive_bob),
            Self::convert_to_index(leave_bob),
        );
        let (day1, day2) = if alice.0 < bob.0 {
            (alice, bob)
        } else {
            (bob, alice)
        };
        if day1.1 < day2.0 {
            0
        } else {
            day1.1.min(day2.1) - day2.0 + 1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2409() {
        assert_eq!(
            Solution::count_days_together(
                "08-15".to_string(),
                "08-18".to_string(),
                "08-16".to_string(),
                "08-19".to_string()
            ),
            3
        );
        assert_eq!(
            Solution::count_days_together(
                "10-01".to_string(),
                "10-31".to_string(),
                "11-01".to_string(),
                "12-31".to_string()
            ),
            0
        );
    }
}
