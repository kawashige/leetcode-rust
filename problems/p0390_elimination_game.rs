pub struct Solution {}

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        let mut front = false;
        let mut step = 2;
        let mut min = 2;
        let mut max = if n % 2 == 0 { n } else { n - 1 };

        while min < max {
            let count = 1 + (max - min) / step;

            if front {
                min += step;
                if count % 2 == 1 {
                    max -= step;
                }
            } else {
                max -= step;
                if count % 2 == 1 {
                    min += step;
                }
            }
            step *= 2;
            front = !front;
        }
        min
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0390() {
        assert_eq!(54, Solution::last_remaining(100));
        assert_eq!(534765398, Solution::last_remaining(1000000000));
        assert_eq!(1, Solution::last_remaining(1));
        assert_eq!(2, Solution::last_remaining(2));
        assert_eq!(2, Solution::last_remaining(3));
        assert_eq!(6, Solution::last_remaining(9));
        assert_eq!(8, Solution::last_remaining(10));
    }
}
