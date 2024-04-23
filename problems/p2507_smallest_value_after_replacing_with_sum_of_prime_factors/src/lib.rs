pub struct Solution {}

impl Solution {
    pub fn smallest_value(n: i32) -> i32 {
        let mut sum = 0;
        let mut remains = n;
        for i in 2..=n {
            while remains % i == 0 {
                remains /= i;
                sum += i;
                if remains == 1 {
                    break;
                }
            }
        }
        if n != sum {
            Self::smallest_value(sum)
        } else {
            n
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2507() {
        assert_eq!(Solution::smallest_value(15), 5);
        assert_eq!(Solution::smallest_value(3), 3);
    }
}
