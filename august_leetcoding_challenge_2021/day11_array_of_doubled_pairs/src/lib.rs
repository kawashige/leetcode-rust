use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn can_reorder_doubled(mut arr: Vec<i32>) -> bool {
        const BASE: i32 = 200_000;
        arr.sort_unstable_by_key(|a| (a.signum(), a.abs()));
        arr.into_iter()
            .fold(
                (vec![0; BASE as usize * 2 + 1], 0),
                |(mut count, mut remains), x| {
                    if count[(x + BASE) as usize] > 0 {
                        count[(x + BASE) as usize] -= 1;
                        remains -= 1;
                    } else {
                        count[(x * 2 + BASE) as usize] += 1;
                        remains += 1;
                    }
                    (count, remains)
                },
            )
            .1
            == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day11() {
        assert!(!Solution::can_reorder_doubled(vec![100000, -100000]));
        assert!(!Solution::can_reorder_doubled(vec![3, 1, 3, 6]));
        assert!(!Solution::can_reorder_doubled(vec![2, 1, 2, 6]));
        assert!(Solution::can_reorder_doubled(vec![4, -2, 2, -4]));
        assert!(!Solution::can_reorder_doubled(vec![1, 2, 4, 16, 8, 4]));
    }
}
