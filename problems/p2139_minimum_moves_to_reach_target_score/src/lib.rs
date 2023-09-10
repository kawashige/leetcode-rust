pub struct Solution {}

impl Solution {
    pub fn min_moves(target: i32, max_doubles: i32) -> i32 {
        let mut count = 0;
        let mut current = target;
        let mut doubles = max_doubles;

        while 1 < current {
            println!(
                "current: {}, count: {}, doubles: {}",
                current, count, doubles
            );
            if 0 < doubles {
                if current % 2 == 1 {
                    current -= 1;
                    count += 1;
                } else {
                    current /= 2;
                    doubles -= 1;
                    count += 1;
                }
            } else {
                count += current - 1;
                current = 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2139() {
        assert_eq!(Solution::min_moves(5, 0), 4);
        assert_eq!(Solution::min_moves(19, 2), 7);
        assert_eq!(Solution::min_moves(10, 4), 4);
    }
}
