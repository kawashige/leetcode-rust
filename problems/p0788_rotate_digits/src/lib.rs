pub struct Solution {}

impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        (1..=n)
            .filter(|i| {
                let s = i.to_string();
                let l = s.len();
                let (j, k) = s.chars().fold((0, 0), |(j, k), c| match c {
                    '0' | '1' | '8' => (j + 1, k),
                    '2' | '5' | '6' | '9' => (j, k + 1),
                    _ => (j, k),
                });
                k > 0 && j + k == l
            })
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0788() {
        assert_eq!(Solution::rotated_digits(857), 247);
        assert_eq!(Solution::rotated_digits(10), 4);
    }
}
