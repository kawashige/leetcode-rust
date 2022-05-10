pub struct Solution {}

impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut count = [0; 5];

        for b in croak_of_frogs.as_bytes() {
            let index = match b {
                b'c' => 0,
                b'r' => 1,
                b'o' => 2,
                b'a' => 3,
                b'k' => 4,
                _ => unreachable!(),
            };
            if 0 < index && count[index - 1] == 0 {
                return -1;
            }
            if 0 < count[(5 + index - 1) % 5] {
                count[(5 + index - 1) % 5] -= 1;
            }
            count[index] += 1;
        }

        if count[..4].iter().any(|c| &0 < c) {
            -1
        } else {
            count[4]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1419() {
        assert_eq!(Solution::min_number_of_frogs("croakcroak".to_string()), 1);
        assert_eq!(Solution::min_number_of_frogs("crcoakroak".to_string()), 2);
        assert_eq!(Solution::min_number_of_frogs("croakcrook".to_string()), -1);
    }
}
