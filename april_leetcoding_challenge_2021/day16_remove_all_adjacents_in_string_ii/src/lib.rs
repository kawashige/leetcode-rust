pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        s.chars()
            .fold(Vec::new(), |mut stack: Vec<(char, i32)>, c| {
                if !stack.is_empty() && stack.last().unwrap().0 == c {
                    if stack.last().unwrap().1 == k - 1 {
                        stack.pop();
                    } else {
                        stack.last_mut().unwrap().1 += 1;
                    }
                } else {
                    stack.push((c, 1));
                }
                stack
            })
            .into_iter()
            .map(|(c, n)| std::iter::repeat(c).take(n as usize).collect::<String>())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day16() {
        assert_eq!(
            Solution::remove_duplicates("abcd".to_string(), 2),
            "abcd".to_string()
        );
        assert_eq!(
            Solution::remove_duplicates("deeedbbcccbdaa".to_string(), 3),
            "aa".to_string()
        );
        assert_eq!(
            Solution::remove_duplicates("pbbcggttciiippooaais".to_string(), 2),
            "ps".to_string()
        );
    }
}
