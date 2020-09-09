pub struct Solution {}

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1 = version1
            .split('.')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let v2 = version2
            .split('.')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if v1.len() > v2.len() {
            for (n1, n2) in v1
                .into_iter()
                .zip(v2.into_iter().chain(std::iter::repeat(0)))
            {
                if n1 < n2 {
                    return -1;
                } else if n1 > n2 {
                    return 1;
                }
            }
        } else {
            for (n1, n2) in v1
                .into_iter()
                .chain(std::iter::repeat(0))
                .zip(v2.into_iter())
            {
                if n1 < n2 {
                    return -1;
                } else if n1 > n2 {
                    return 1;
                }
            }
        }
        0
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day9() {
        assert_eq!(
            -1,
            Solution::compare_version("7.5.2.4".to_string(), "7.5.3".to_string())
        );
        assert_eq!(
            1,
            Solution::compare_version("1.0.1".to_string(), "1".to_string())
        );
        assert_eq!(
            -1,
            Solution::compare_version("7.5.2.4".to_string(), "7.5.3".to_string())
        );
        assert_eq!(
            0,
            Solution::compare_version("1.01".to_string(), "1.001".to_string())
        );
        assert_eq!(
            0,
            Solution::compare_version("1.0".to_string(), "1.0.0".to_string())
        );
        assert_eq!(
            -1,
            Solution::compare_version("1".to_string(), "1.1".to_string())
        );
    }
}
