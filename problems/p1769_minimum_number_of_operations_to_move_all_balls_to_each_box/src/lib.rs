pub struct Solution {}

impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let boxes = boxes.as_bytes();

        let mut left_to_right = vec![0; boxes.len()];
        let mut count = 0;
        for i in 1..boxes.len() {
            count += (boxes[i - 1] - b'0') as i32;
            left_to_right[i] = count + left_to_right[i - 1];
        }

        let mut right_to_left = vec![0; boxes.len()];
        let mut count = 0;
        for i in (0..boxes.len() - 1).rev() {
            count += (boxes[i + 1] - b'0') as i32;
            right_to_left[i] = count + right_to_left[i + 1];
        }

        left_to_right
            .into_iter()
            .zip(right_to_left.into_iter())
            .map(|(l, r)| l + r)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1769() {
        assert_eq!(Solution::min_operations("110".to_string()), [1, 1, 3]);
        assert_eq!(
            Solution::min_operations("001011".to_string()),
            [11, 8, 5, 4, 3, 4]
        );
    }
}
