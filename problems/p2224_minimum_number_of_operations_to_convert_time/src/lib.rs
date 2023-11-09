pub struct Solution {}

impl Solution {
    pub fn to_mininutes(t: String) -> i32 {
        let mut v = t.split(':');
        v.next().unwrap().parse::<i32>().unwrap() * 60 + v.next().unwrap().parse::<i32>().unwrap()
    }
    pub fn convert_time(current: String, correct: String) -> i32 {
        let mut remains = Self::to_mininutes(correct) - Self::to_mininutes(current);
        let mut count = 0;

        for inc in [60, 15, 5, 1].iter() {
            count += remains / inc;
            remains %= inc;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2224() {
        assert_eq!(
            Solution::convert_time("02:30".to_string(), "04:35".to_string()),
            3
        );
        assert_eq!(
            Solution::convert_time("11:00".to_string(), "11:01".to_string()),
            1
        );
    }
}
