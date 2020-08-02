pub struct Solution {}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        fn count(n: String) -> String {
            let mut chars = n.chars();
            let mut value = chars.next().unwrap();
            let mut count = 1;
            let mut result = Vec::new();

            while let Some(c) = chars.next() {
                if c != value {
                    result.push(count.to_string());
                    result.push(value.to_string());
                    value = c;
                    count = 1;
                } else {
                    count += 1;
                }
            }

            result.push(count.to_string());
            result.push(value.to_string());

            result.into_iter().collect::<String>()
        }

        let mut result = "1".to_string();
        for _ in 1..n {
            result = count(result);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_38() {
        assert_eq!("1211", Solution::count_and_say(4));
    }
}
