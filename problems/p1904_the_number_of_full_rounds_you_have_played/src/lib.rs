pub struct Solution {}

impl Solution {
    pub fn time_to_minutes(time: &str) -> i32 {
        let mut sp = time.split(":");
        sp.next().unwrap().parse::<i32>().unwrap() * 60 + sp.next().unwrap().parse::<i32>().unwrap()
    }

    pub fn number_of_rounds(login_time: String, logout_time: String) -> i32 {
        let mut start = Self::time_to_minutes(&login_time);
        let mut end = Self::time_to_minutes(&logout_time);
        if end <= start {
            end += 24 * 60;
        }
        start += (15 - start % 15) % 15;
        end -= end % 15;

        ((end - start) / 15).max(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1904() {
        assert_eq!(
            Solution::number_of_rounds("0047:46".to_string(), "00:57".to_string()),
            0
        );
        assert_eq!(
            Solution::number_of_rounds("04:54".to_string(), "18:51".to_string()),
            55
        );
        assert_eq!(
            Solution::number_of_rounds("09:31".to_string(), "10:14".to_string()),
            1
        );
        assert_eq!(
            Solution::number_of_rounds("21:30".to_string(), "03:00".to_string()),
            22
        );
    }
}
