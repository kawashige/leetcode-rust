pub struct Solution {}

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..colors.len() {
            if colors[i] != colors[(i + 1) % colors.len()]
                && colors[i] != colors[(i + colors.len() - 1) % colors.len()]
            {
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
    fn test_3206() {
        assert_eq!(Solution::number_of_alternating_groups(vec![1, 1, 1]), 0);
        assert_eq!(
            Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1]),
            3
        );
    }
}
