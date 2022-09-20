pub struct Solution {}

impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        let mut remains = n;
        let mut count = 0;
        while 1 < remains {
            count += remains / 2;
            remains = remains / 2 + remains % 2;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1688() {
        assert_eq!(Solution::number_of_matches(7), 6);
        assert_eq!(Solution::number_of_matches(14), 13);
    }
}
