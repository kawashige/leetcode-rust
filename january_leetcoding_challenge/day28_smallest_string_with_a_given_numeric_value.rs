pub struct Solution {}

impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        (1..27)
            .rev()
            .scan((k, n), |(k, n), i| {
                // println!("k: {}, n: {}, i: {}", k, n, i);
                let mut s = String::new();
                while 0 < *n && *k - i >= *n - 1 {
                    *k -= i;
                    *n -= 1;
                    s.push(char::from(0x61 + (i - 1) as u8));
                }
                Some(s)
            })
            .collect::<Vec<String>>()
            .into_iter()
            .rev()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day28() {
        assert_eq!(Solution::get_smallest_string(5, 130), "zzzzz".to_string());
        assert_eq!(Solution::get_smallest_string(3, 27), "aay".to_string());
        assert_eq!(Solution::get_smallest_string(5, 73), "aaszz".to_string());
    }
}
