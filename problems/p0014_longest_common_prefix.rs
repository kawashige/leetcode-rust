pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".to_string();
        }
        let max = strs[0].len();
        for i in 0..max {
            for j in 1..strs.len() {
                if strs[j].len() < i + 1 || strs[0][0..(i + 1)] != strs[j][0..(i + 1)] {
                    return strs[0][0..i].to_string();
                }
            }
        }
        strs[0].clone()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_14() {
        let ret = Solution::longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ]);
        assert_eq!("fl", ret);
    }
}
