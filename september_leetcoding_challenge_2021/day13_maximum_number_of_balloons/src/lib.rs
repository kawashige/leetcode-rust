pub struct Solution {}

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let count = text.chars().fold([0; 26], |mut count, c| {
            count[c as usize - 0x61] += 1;
            count
        });

        [('b', 1), ('a', 1), ('l', 2), ('o', 2), ('n', 1)]
            .iter()
            .map(|(c, i)| count[*c as usize - 0x61] / i)
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day13() {
        assert_eq!(Solution::max_number_of_balloons("nlaebolko".to_string()), 1);
        assert_eq!(
            Solution::max_number_of_balloons("loonbalxballpoon".to_string()),
            2
        );
        assert_eq!(Solution::max_number_of_balloons("leetcode".to_string()), 0);
    }
}
