pub struct Solution {}

impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let mut i = 0;
        while i < arr.len() {
            if let Some(p) = pieces.iter().find(|p| p[0] == arr[i]) {
                if (1..p.len()).any(|j| arr[i + j] != p[j]) {
                    return false;
                }
                i += p.len();
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day1() {
        assert!(Solution::can_form_array(vec![85], vec![vec![85]]));
        assert!(Solution::can_form_array(
            vec![15, 88],
            vec![vec![88], vec![15]]
        ));
        assert!(!Solution::can_form_array(
            vec![49, 18, 16],
            vec![vec![16, 18, 19]]
        ));
        assert!(Solution::can_form_array(
            vec![91, 4, 64, 78],
            vec![vec![78], vec![4, 64], vec![91]]
        ));
        assert!(!Solution::can_form_array(
            vec![1, 3, 5, 7],
            vec![vec![2, 4, 6, 8]]
        ));
        assert!(Solution::can_form_array(
            vec![2, 82, 79, 95, 28],
            vec![vec![2], vec![82], vec![28], vec![79, 95]]
        ));
    }
}
