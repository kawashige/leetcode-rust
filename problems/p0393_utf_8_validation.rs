pub struct Solution {}

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        fn valid_succeed_byte(b: i32) -> bool {
            b & (1 << 7) != 0 && b & (1 << 6) == 0
        }
        fn recurse(data: &[i32]) -> bool {
            if data.is_empty() {
                return true;
            }
            if data[0] & (1 << 7) == 0 {
                return recurse(&data[1..]);
            } else {
                if data[0] & (1 << 6) != 0 {
                    if data[0] & (1 << 5) == 0 {
                        return 1 < data.len()
                            && valid_succeed_byte(data[1])
                            && recurse(&data[2..]);
                    } else {
                        if data[0] & (1 << 4) == 0 {
                            return 2 < data.len()
                                && valid_succeed_byte(data[1])
                                && valid_succeed_byte(data[2])
                                && recurse(&data[3..]);
                        } else if data[0] & (1 << 3) == 0 {
                            return 3 < data.len()
                                && valid_succeed_byte(data[1])
                                && valid_succeed_byte(data[2])
                                && valid_succeed_byte(data[3])
                                && recurse(&data[4..]);
                        }
                    }
                }
            }
            false
        }
        recurse(&data)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0393() {
        assert!(!Solution::valid_utf8(vec![237]));
        assert!(Solution::valid_utf8(vec![197, 130, 1]));
        assert!(!Solution::valid_utf8(vec![235, 140, 4]));
    }
}
