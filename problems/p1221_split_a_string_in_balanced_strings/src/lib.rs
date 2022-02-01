pub struct Solution {}

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut amount = 0;
        let mut count = 0;

        s.chars().for_each(|c| {
            count += if c == 'R' { 1 } else { -1 };
            if count == 0 {
                amount += 1;
            }
        });

        amount
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1221() {
        assert_eq!(Solution::balanced_string_split("RLRRLLRLRL".to_string()), 4);
        assert_eq!(Solution::balanced_string_split("RLLLLRRRLR".to_string()), 3);
        assert_eq!(Solution::balanced_string_split("LLLLRRRR".to_string()), 1);
    }
}
