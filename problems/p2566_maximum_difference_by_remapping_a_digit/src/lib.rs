pub struct Solution {}

impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let min_c = num.to_string().chars().nth(0).unwrap();
        let max_c = num.to_string().chars().find(|c| c != &'9').unwrap_or('9');

        num.to_string()
            .chars()
            .map(|c| if c == max_c { '9' } else { c })
            .collect::<String>()
            .parse::<i32>()
            .unwrap()
            - num
                .to_string()
                .chars()
                .map(|c| if c == min_c { '0' } else { c })
                .collect::<String>()
                .parse::<i32>()
                .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2566() {
        assert_eq!(Solution::min_max_difference(11891), 99009);
        assert_eq!(Solution::min_max_difference(90), 99);
    }
}
