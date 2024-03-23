pub struct Solution {}

impl Solution {
    pub fn convert(t: &str) -> usize {
        let v = t
            .split(':')
            .filter_map(|sp| sp.parse::<usize>().ok())
            .collect::<Vec<_>>();
        v[0] * 60 + v[1]
    }

    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        let t1 = (Self::convert(&event1[0]), Self::convert(&event1[1]));
        let t2 = (Self::convert(&event2[0]), Self::convert(&event2[1]));

        if t1.0 < t2.0 {
            t2.0 <= t1.1
        } else {
            t1.0 <= t2.1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2446() {
        assert!(Solution::have_conflict(
            vec!["01:15".to_string(), "02:00".to_string()],
            vec!["02:00".to_string(), "03:00".to_string()]
        ));
        assert!(Solution::have_conflict(
            vec!["01:00".to_string(), "02:00".to_string()],
            vec!["01:20".to_string(), "03:00".to_string()]
        ));
        assert!(!Solution::have_conflict(
            vec!["10:00".to_string(), "11:00".to_string()],
            vec!["14:00".to_string(), "15:00".to_string()]
        ));
    }
}
