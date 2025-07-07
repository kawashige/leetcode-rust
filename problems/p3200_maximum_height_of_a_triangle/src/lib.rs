pub struct Solution {}

impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        let mut result = 1;

        let mut balls = [red, blue];
        for i in 1.. {
            balls[i % 2] -= i as i32;
            if balls[i % 2] < 0 {
                result = result.max(i - 1);
                break;
            }
        }

        let mut balls = [blue, red];
        for i in 1.. {
            balls[i % 2] -= i as i32;
            if balls[i % 2] < 0 {
                result = result.max(i - 1);
                break;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3200() {
        assert_eq!(Solution::max_height_of_triangle(2, 4), 3);
        assert_eq!(Solution::max_height_of_triangle(2, 1), 1);
        assert_eq!(Solution::max_height_of_triangle(1, 1), 1);
        assert_eq!(Solution::max_height_of_triangle(10, 1), 2);
    }
}
