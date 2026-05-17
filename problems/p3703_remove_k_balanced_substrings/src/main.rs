pub struct Solution {}

impl Solution {
    pub fn remove_substring(s: String, k: i32) -> String {
        let mut stack: Vec<(char, i32)> = Vec::new();

        for c in s.chars() {
            if !stack.is_empty() && stack.last().unwrap().0 == c {
                stack.last_mut().unwrap().1 += 1;
            } else {
                stack.push((c, 1));
            }
            if c == ')'
                && 1 < stack.len()
                && k <= stack[stack.len() - 1].1
                && k <= stack[stack.len() - 2].1
            {
                stack.pop();
                stack.last_mut().unwrap().1 -= k;
                if stack.last().unwrap().1 == 0 {
                    stack.pop();
                }
            }
        }

        stack
            .into_iter()
            .map(|(c, i)| std::iter::repeat(c).take(i as usize))
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3703() {
        assert_eq!(
            Solution::remove_substring("(())".to_string(), 1),
            "".to_string()
        );
        assert_eq!(
            Solution::remove_substring("(()(".to_string(), 1),
            "((".to_string()
        );
        assert_eq!(
            Solution::remove_substring("((()))()()()".to_string(), 3),
            "()()()".to_string()
        );
    }
}

fn main() {}
