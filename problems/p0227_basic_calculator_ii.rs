pub struct Solution {}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut tmp = "".to_string();
        let mut nums = Vec::new();
        let mut ops = Vec::new();
        for c in s.chars() {
            match c {
                ' ' => {}
                '+' | '-' | '*' | '/' => {
                    nums.push(tmp.parse::<i32>().unwrap());
                    tmp.clear();
                    ops.push(c);
                }
                _ => tmp.push(c),
            }
        }
        nums.push(tmp.parse::<i32>().unwrap());

        for op in &[['*', '/'], ['+', '-']] {
            while ops.contains(&op[0]) || ops.contains(&op[1]) {
                let i = ops.iter().position(|o| op.contains(o)).unwrap();
                match ops[i] {
                    '*' => nums[i] *= nums[i + 1],
                    '/' => nums[i] /= nums[i + 1],
                    '+' => nums[i] += nums[i + 1],
                    '-' => nums[i] -= nums[i + 1],
                    _ => unreachable!(),
                }
                ops.remove(i);
                nums.remove(i + 1);
            }
        }
        nums[0]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0227() {
        assert_eq!(1, Solution::calculate("1-1+1".to_string()));
        assert_eq!(7, Solution::calculate("3+2*2".to_string()));
        assert_eq!(1, Solution::calculate(" 3/2 ".to_string()));
        assert_eq!(5, Solution::calculate(" 3+5 / 2 ".to_string()));
    }
}
