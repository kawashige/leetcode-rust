pub struct Solution {}

impl Solution {
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut candle_count = vec![0; s.len()];
        let mut left_candle = vec![-1; s.len()];
        let mut candle = -1;
        for i in 0..s.len() {
            if 0 < i {
                candle_count[i] += candle_count[i - 1];
            }
            if s.as_bytes()[i] == b'|' {
                candle = i as i32;
                candle_count[i] += 1;
            }
            left_candle[i] = candle;
        }

        let mut right_candle = vec![-1; s.len()];
        let mut candle = -1;
        for i in (0..s.len()).rev() {
            if s.as_bytes()[i] == b'|' {
                candle = i as i32;
            }
            right_candle[i] = candle;
        }

        queries
            .into_iter()
            .map(|q| {
                if left_candle[q[1] as usize] == -1
                    || candle_count[q[1] as usize] - candle_count[q[0] as usize] == 0
                {
                    0
                } else {
                    left_candle[q[1] as usize]
                        - right_candle[q[0] as usize]
                        - (candle_count[left_candle[q[1] as usize] as usize]
                            - candle_count[right_candle[q[0] as usize] as usize])
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2055() {
        assert_eq!(
            Solution::plates_between_candles(
                "**|**|***|".to_string(),
                vec![vec![2, 5], vec![5, 9]]
            ),
            vec![2, 3]
        );
        assert_eq!(
            Solution::plates_between_candles(
                "***|**|*****|**||**|*".to_string(),
                vec![
                    vec![1, 17],
                    vec![4, 5],
                    vec![14, 17],
                    vec![5, 11],
                    vec![15, 16]
                ]
            ),
            vec![9, 0, 0, 0, 0]
        );
    }
}
