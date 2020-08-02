pub struct Solution {}

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;
        let max = digits.len();
        for i in 0..max {
            if carry == 0 {
                break;
            }
            if digits[max - 1 - i] == 9 {
                digits[max - 1 - i] = 0;
                carry = 1;
            } else {
                digits[max - 1 - i] += 1;
                carry = 0;
            }
        }
        if carry == 1 {
            digits.insert(0, 1);
        }
        digits
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_66() {
        assert_eq!(vec![1, 2, 4], Solution::plus_one(vec![1, 2, 3]))
    }
}
