pub struct Solution {}

impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        let chars = num
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<_>>();
        let mut max = std::i32::MIN;
        let mut min = std::i32::MAX;

        for x in 0..10 {
            for y in 0..10 {
                if y == 0 && chars[0] == x {
                    continue;
                }
                let replaced = chars
                    .iter()
                    .fold(0, |acc, c| acc * 10 + if c == &x { &y } else { c });
                max = max.max(replaced);
                min = min.min(replaced);
            }
        }

        max - min
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1432() {
        assert_eq!(Solution::max_diff(123456), 820000);
        assert_eq!(Solution::max_diff(555), 888);
        assert_eq!(Solution::max_diff(9), 8);
    }
}
