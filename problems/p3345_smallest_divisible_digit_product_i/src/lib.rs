pub struct Solution {}

impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        (n..)
            .find(|x| {
                x.to_string()
                    .chars()
                    .fold(1, |acc, c| acc * (c as u8 - b'0') as i32)
                    % t
                    == 0
            })
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3345() {
        assert_eq!(Solution::smallest_number(10, 2), 10);
        assert_eq!(Solution::smallest_number(15, 3), 16);
    }
}
