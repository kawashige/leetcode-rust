pub struct Solution {}

impl Solution {
    pub fn read_binary_watch(num: i32) -> Vec<String> {
        let mut results = Vec::new();
        if num < 0 || 8 < num {
            return results;
        }

        for i in 0..=3 {
            if num - i < 0 || 5 < num - i {
                continue;
            }
            let hours = (0..12)
                .filter(|n| (*n as i32).count_ones() == i as u32)
                .collect::<Vec<i32>>();
            let minutes = (0..=59)
                .filter(|n| (*n as i32).count_ones() == (num - i) as u32)
                .collect::<Vec<i32>>();

            for h in hours {
                for m in &minutes {
                    results.push(format!("{}:{:02}", h, m));
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0401() {
        assert_eq!(
            vec![
                "0:01".to_string(),
                "0:02".to_string(),
                "0:04".to_string(),
                "0:08".to_string(),
                "0:16".to_string(),
                "0:32".to_string(),
                "1:00".to_string(),
                "2:00".to_string(),
                "4:00".to_string(),
                "8:00".to_string(),
            ],
            Solution::read_binary_watch(1)
        );
        assert_eq!(vec![] as Vec<String>, Solution::read_binary_watch(10));
        assert_eq!(vec![] as Vec<String>, Solution::read_binary_watch(-1));
        assert_eq!(
            vec!["0:00".to_string()] as Vec<String>,
            Solution::read_binary_watch(0)
        );
    }
}
