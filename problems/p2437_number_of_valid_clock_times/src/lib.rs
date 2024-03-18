pub struct Solution {}

impl Solution {
    pub fn count_time(time: String) -> i32 {
        let chars = time.chars().collect::<Vec<_>>();
        let mut result = 0;

        for h in 0..24 {
            let h = format!("{:0>2}", h).chars().collect::<Vec<_>>();
            if (chars[0] != '?' && chars[0] != h[0]) || (chars[1] != '?' && chars[1] != h[1]) {
                continue;
            }
            for m in 0..60 {
                let m = format!("{:0>2}", m).chars().collect::<Vec<_>>();
                if (chars[3] != '?' && chars[3] != m[0]) || (chars[4] != '?' && chars[4] != m[1]) {
                    continue;
                }
                result += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2437() {
        assert_eq!(Solution::count_time("?5:00".to_string()), 2);
        assert_eq!(Solution::count_time("0?:0?".to_string()), 100);
        assert_eq!(Solution::count_time("??:??".to_string()), 1440);
    }
}
