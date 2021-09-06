pub struct Solution {}

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        release_times
            .iter()
            .zip(keys_pressed.chars())
            .enumerate()
            .fold(None, |max, (i, (t, c))| {
                if max.is_none() {
                    Some((*t, c))
                } else if max.unwrap() < (t - release_times[i - 1], c) {
                    Some((t - release_times[i - 1], c))
                } else {
                    max
                }
            })
            .unwrap()
            .1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day06() {
        assert_eq!(
            Solution::slowest_key(vec![9, 29, 49, 50], "cbcd".to_string()),
            'c'
        );
        assert_eq!(
            Solution::slowest_key(vec![12, 23, 36, 46, 62], "spuda".to_string()),
            'a'
        );
    }
}
