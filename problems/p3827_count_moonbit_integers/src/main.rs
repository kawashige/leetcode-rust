pub struct Solution {}

impl Solution {
    pub fn count_monobit(n: i32) -> i32 {
        let mut count = 1;
        let mut x = 0;
        while (x << 1 | 1) <= n {
            count += 1;
            x = x << 1 | 1;
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3827() {
        assert_eq!(Solution::count_monobit(1), 2);
        assert_eq!(Solution::count_monobit(4), 3);
    }
}

fn main() {}
