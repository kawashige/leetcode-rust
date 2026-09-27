pub struct Solution {}

impl Solution {
    pub fn min_cost(n: i32) -> i32 {
        if n == 1 {
            0
        } else {
            n - 1 + Self::min_cost(n - 1)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3857() {
        assert_eq!(Solution::min_cost(3), 3);
        assert_eq!(Solution::min_cost(4), 6);
    }
}

fn main() {}
