pub struct Solution {}

impl Solution {
    pub fn is_fascinating(n: i32) -> bool {
        let mut count = [0; 10];
        for x in [n, n * 2, n * 3].iter() {
            let mut x = *x;
            while 0 < x {
                count[(x % 10) as usize] += 1;
                x /= 10;
            }
        }
        count[0] == 0 && count[1..].iter().all(|c| c == &1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2729() {
        assert!(Solution::is_fascinating(192));
        assert!(!Solution::is_fascinating(100));
    }
}
