pub struct Solution {}

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        if k == 1 {
            let mut chars = s.chars().collect::<Vec<char>>();
            let mut r = s;
            for _ in 1..chars.len() {
                chars.rotate_left(1);
                let new_s = chars.iter().collect();
                if r > new_s {
                    r = new_s;
                }
            }
            r
        } else {
            let mut chars = s.chars().collect::<Vec<char>>();
            chars.sort_unstable();
            chars.into_iter().collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day05() {
        assert_eq!(
            Solution::orderly_queue("cba".to_string(), 1),
            "acb".to_string()
        );
        assert_eq!(
            Solution::orderly_queue("baaca".to_string(), 3),
            "aaabc".to_string()
        );
    }
}
