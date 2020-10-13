pub struct Solution {}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return Vec::new();
        }

        let mut start = nums[0];
        let mut previous = start;
        let mut results = Vec::new();
        for i in 1..nums.len() {
            if previous + 1 < nums[i] {
                results.push(if start == previous {
                    start.to_string()
                } else {
                    format!("{}->{}", start, previous)
                });
                start = nums[i];
                previous = start;
            } else {
                previous = nums[i]
            }
        }
        results.push(if start == previous {
            start.to_string()
        } else {
            format!("{}->{}", start, previous)
        });

        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0228() {
        assert_eq!(
            vec!["0->2".to_string(), "4->5".to_string(), "7".to_string()],
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7])
        );
        assert_eq!(
            vec![
                "0".to_string(),
                "2->4".to_string(),
                "6".to_string(),
                "8->9".to_string()
            ],
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9])
        );
        assert_eq!(vec![] as Vec<String>, Solution::summary_ranges(vec![]));
        assert_eq!(vec!["-1".to_string()], Solution::summary_ranges(vec![-1]));
    }
}
