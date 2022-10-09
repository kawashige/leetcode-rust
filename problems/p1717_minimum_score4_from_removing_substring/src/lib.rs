pub struct Solution {}

impl Solution {
    pub fn calculate_score(stack: &Vec<u8>, x: i32, y: i32) -> i32 {
        let mut score = 0;
        let mut new_stack = Vec::with_capacity(stack.len());

        for b in stack {
            match (b, new_stack.last()) {
                (b'a', Some(&b'b')) => {
                    score += y;
                    new_stack.pop();
                }
                (b'b', Some(&b'a')) => {
                    score += x;
                    new_stack.pop();
                }
                _ => {
                    new_stack.push(*b);
                }
            }
        }
        println!("{}", score);

        score
    }

    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let mut score = 0;
        let mut stack = Vec::with_capacity(s.len());

        for b in s.as_bytes() {
            match (b, stack.last()) {
                (&b'a', Some(&b'b')) if x < y => {
                    score += y;
                    stack.pop();
                }
                (&b'b', Some(&b'a')) if y < x => {
                    score += x;
                    stack.pop();
                }
                (&b'a', _) | (&b'b', _) => {
                    stack.push(*b);
                }
                _ => {
                    score += Self::calculate_score(&stack, x, y);
                    stack.clear();
                }
            }
        }

        score + Self::calculate_score(&stack, x, y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1717() {
        assert_eq!(
            Solution::maximum_gain("cbaabwbbbabbwaaq".to_string(), 4074, 9819),
            23712
        );
        assert_eq!(Solution::maximum_gain("cdbcbbaaabab".to_string(), 4, 5), 19);
        assert_eq!(
            Solution::maximum_gain("aabbaaxybbaabb".to_string(), 5, 4),
            20
        );
    }
}
