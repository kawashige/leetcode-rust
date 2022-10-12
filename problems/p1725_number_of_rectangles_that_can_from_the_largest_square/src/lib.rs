pub struct Solution {}

impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let mut max_len = 0;

        for rectangle in rectangles {
            let len = rectangle[0].min(rectangle[1]);
            if max_len < len {
                max_len = len;
                count = 1;
            } else if max_len == len {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1725() {
        assert_eq!(
            Solution::count_good_rectangles(vec![vec![5, 8], vec![3, 9], vec![5, 12], vec![16, 5]]),
            3
        );
        assert_eq!(
            Solution::count_good_rectangles(vec![vec![2, 3], vec![3, 7], vec![4, 3], vec![3, 7]]),
            3
        );
    }
}
