pub struct Solution {}

impl Solution {
    pub fn minimize_result(expression: String) -> String {
        let nums = expression.split('+').collect::<Vec<_>>();

        let mut min_result = std::i32::MAX;
        let mut min_l = 0;
        let mut min_r = 0;

        for l in 0..nums[0].len() {
            for r in 0..nums[1].len() {
                let result = nums[0][..l].parse::<i32>().unwrap_or(1)
                    * (nums[0][l..].parse::<i32>().unwrap()
                        + nums[1][..=r].parse::<i32>().unwrap())
                    * nums[1][r + 1..].parse::<i32>().unwrap_or(1);
                if result < min_result {
                    min_result = result;
                    min_l = l;
                    min_r = r;
                }
            }
        }

        format!(
            "{}({}+{}){}",
            if min_l == 0 {
                "".to_string()
            } else {
                nums[0][..min_l].to_string()
            },
            nums[0][min_l..].to_string(),
            nums[1][..=min_r].to_string(),
            if min_r == nums[1].len() - 1 {
                "".to_string()
            } else {
                nums[1][min_r + 1..].to_string()
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2232() {
        assert_eq!(
            Solution::minimize_result("247+38".to_owned()),
            "2(47+38)".to_string()
        );
        assert_eq!(
            Solution::minimize_result("12+34".to_string()),
            "1(2+3)4".to_string()
        );
        assert_eq!(
            Solution::minimize_result("999+999".to_string()),
            "(999+999)".to_string()
        );
    }
}
