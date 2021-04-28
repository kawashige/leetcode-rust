pub struct Solution {}

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        s.chars().fold(vec![1, 0], |v, c| {
            if v[1] + widths[c as usize - 0x61] > 100 {
                vec![v[0] + 1, widths[c as usize - 0x61]]
            } else {
                vec![v[0], v[1] + widths[c as usize - 0x61]]
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0806() {
        assert_eq!(
            Solution::number_of_lines(
                vec![
                    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                "abcdefghijklmnopqrstuvwxyz".to_string()
            ),
            vec![3, 60]
        );

        assert_eq!(
            Solution::number_of_lines(
                vec![
                    4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                "bbbcccdddaaa".to_string()
            ),
            vec![2, 4]
        );
    }
}
