use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn recurse(a: i32, b: i32, memo: &mut HashMap<(i32, i32), f64>) -> f64 {
        if let Some(p) = memo.get(&(a, b)) {
            *p
        } else if a == 0 && b > 0 {
            1.0
        } else if a == 0 && b == 0 {
            0.5
        } else if b == 0 {
            0.0
        } else {
            let r = 0.25
                * (Self::recurse(std::cmp::max(0, a - 100), b, memo)
                    + Self::recurse(std::cmp::max(0, a - 75), std::cmp::max(0, b - 25), memo)
                    + Self::recurse(std::cmp::max(0, a - 50), std::cmp::max(0, b - 50), memo)
                    + Self::recurse(std::cmp::max(0, a - 25), std::cmp::max(0, b - 75), memo));
            memo.insert((a, b), r);
            r
        }
    }

    pub fn soup_servings(n: i32) -> f64 {
        Self::recurse(n, n, &mut HashMap::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0808() {
        assert_eq!(Solution::soup_servings(50), 0.625);
    }
}
