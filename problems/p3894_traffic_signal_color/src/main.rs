pub struct Solution {}

impl Solution {
    pub fn traffic_signal(timer: i32) -> String {
        if (31..=90).contains(&timer) {
            "Red".to_string()
        } else if timer == 30 {
            "Orange".to_string()
        } else if timer == 0 {
            "Green".to_string()
        } else {
            "Invalid".to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3894() {
        assert_eq!(Solution::traffic_signal(60), "Red".to_string());
        assert_eq!(Solution::traffic_signal(5), "Invalid".to_string());
    }
}

fn main() {}
