pub struct Solution {}

impl Solution {
    pub fn solve_equation(equation: String) -> String {
        fn calc(s: &str) -> (i32, i32) {
            let mut nums = (0, 0);
            let mut n = None;
            let mut sign: i32 = 1;
            for c in s.chars() {
                match c {
                    '+' | '-' => {
                        if n.is_some() {
                            nums.1 += sign * n.unwrap();
                        }
                        n = None;
                        sign = if c == '+' { 1 } else { -1 };
                    }
                    'x' => {
                        nums.0 += if n.is_none() { sign } else { sign * n.unwrap() };
                        n = None;
                    }
                    _ => {
                        n = Some(n.unwrap_or(0) * 10 + c.to_digit(10).unwrap() as i32);
                    }
                }
            }
            if n.is_some() {
                nums.1 += sign * n.unwrap();
            }
            nums
        }

        let mut es = equation.split('=');
        let lhs = calc(es.next().unwrap());
        let rhs = calc(es.next().unwrap());
        match (lhs.0 - rhs.0, rhs.1 - lhs.1) {
            (0, 0) => "Infinite solutions".to_string(),
            (0, _) => "No solution".to_string(),
            (a, b) => format!("x={}", b / a),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0640() {
        assert_eq!(
            Solution::solve_equation("0x=0".to_string()),
            "Infinite solutions".to_string()
        );
        assert_eq!(
            Solution::solve_equation("x+5-3+x=6+x-2".to_string()),
            "x=2".to_string()
        );
        assert_eq!(
            Solution::solve_equation("x=x".to_string()),
            "Infinite solutions".to_string()
        );
        assert_eq!(
            Solution::solve_equation("2x=x".to_string()),
            "x=0".to_string()
        );
        assert_eq!(
            Solution::solve_equation("2x+3x-6x=x+2".to_string()),
            "x=-1".to_string()
        );
        assert_eq!(
            Solution::solve_equation("x=x+2".to_string()),
            "No solution".to_string()
        );
    }
}
