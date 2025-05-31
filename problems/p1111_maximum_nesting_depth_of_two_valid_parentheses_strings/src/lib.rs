pub struct Solution {}

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut open_count = [0; 2];
        let mut result = vec![0; seq.len()];

        for i in 0..seq.len() {
            if seq.as_bytes()[i] == b'(' {
                let j = (0..2).min_by_key(|j| open_count[*j]).unwrap();
                result[i] = j as i32;
                open_count[j] += 1;
            } else {
                let j = (0..2).max_by_key(|j| open_count[*j]).unwrap();
                result[i] = j as i32;
                open_count[j] -= 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1111() {
        assert_eq!(
            Solution::max_depth_after_split("(()())".to_string()),
            [0, 1, 1, 1, 1, 0]
        );
        assert_eq!(
            Solution::max_depth_after_split("()(())()".to_string()),
            [0, 0, 0, 1, 1, 0, 1, 1]
        );
    }
}
