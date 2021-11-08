pub struct Solution {}

impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        let mut stack = Vec::new();

        for i in (1..=n).rev() {
            match (n - i) % 4 {
                0 if i == n => stack.push(i),
                0 => stack.push(-i),
                1 => *stack.last_mut().unwrap() *= i,
                2 => *stack.last_mut().unwrap() /= i,
                3 => stack.push(i),
                _ => unreachable!(),
            }
        }

        stack.into_iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1006() {
        assert_eq!(Solution::clumsy(4), 7);
        assert_eq!(Solution::clumsy(10), 12);
    }
}
