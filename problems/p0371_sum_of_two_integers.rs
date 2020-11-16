pub struct Solution {}

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut result = 0;
        let mut overflow = false;
        for i in 0..32 {
            let mut ones = Vec::new();
            if overflow {
                ones.push(1);
            }
            if i < 31 {
                if 0 < a & 1 << i {
                    ones.push(1);
                }
                if 0 < b & 1 << i {
                    ones.push(1);
                }
            } else {
                if a < 0 {
                    ones.push(1);
                }
                if b < 0 {
                    ones.push(1);
                }
            }
            let count = ones.len();
            if count == 1 || count == 3 {
                result |= 1 << i;
            }
            overflow = if count < 2 { false } else { true };
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0371() {
        assert_eq!(3, Solution::get_sum(1, 2));
        assert_eq!(1, Solution::get_sum(-2, 3));
        assert_eq!(-10, Solution::get_sum(-3, -7));
    }
}
