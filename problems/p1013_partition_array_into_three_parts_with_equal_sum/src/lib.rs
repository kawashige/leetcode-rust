pub struct Solution {}

impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let acc = arr
            .into_iter()
            .scan(0, |state, x| {
                *state += x;
                Some(*state)
            })
            .collect::<Vec<_>>();

        if acc[acc.len() - 1] % 3 == 0 {
            if let Some(i) = (0..acc.len() - 1)
                .rev()
                .find(|i| acc[*i] == (acc[acc.len() - 1] / 3) * 2)
            {
                if (0..i)
                    .rev()
                    .find(|j| acc[*j] == acc[acc.len() - 1] / 3)
                    .is_some()
                {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1013() {
        assert!(Solution::can_three_parts_equal_sum(vec![
            0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1
        ]));
        assert!(!Solution::can_three_parts_equal_sum(vec![
            0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1
        ]));
        assert!(Solution::can_three_parts_equal_sum(vec![
            3, 3, 6, 5, -2, 2, 5, 1, -9, 4
        ]));
    }
}
