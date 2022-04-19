use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let reserved = reserved_seats
            .into_iter()
            .fold(HashMap::new(), |mut map, reserved| {
                *map.entry(reserved[0]).or_insert(!0_usize) &= !(1 << reserved[1]);
                map
            });

        let mask1 = (2..6).fold(0, |acc, i| acc | 1 << i);
        let mask2 = (4..8).fold(0, |acc, i| acc | 1 << i);
        let mask3 = (6..10).fold(0, |acc, i| acc | 1 << i);
        let mask4 = mask1 | mask2 | mask3;

        (n - reserved.len() as i32) * 2
            + reserved
                .values()
                .into_iter()
                .map(|reserved| {
                    if reserved & mask4 == mask4 {
                        2
                    } else if reserved & mask1 == mask1
                        || reserved & mask2 == mask2
                        || reserved & mask3 == mask3
                    {
                        1
                    } else {
                        0
                    }
                })
                .sum::<i32>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1386() {
        assert_eq!(
            Solution::max_number_of_families(
                3,
                vec![
                    vec![1, 2],
                    vec![1, 3],
                    vec![1, 8],
                    vec![2, 6],
                    vec![3, 1],
                    vec![3, 10]
                ]
            ),
            4
        );
        assert_eq!(
            Solution::max_number_of_families(2, vec![vec![2, 1], vec![1, 8], vec![2, 6]]),
            2
        );
        assert_eq!(
            Solution::max_number_of_families(
                4,
                vec![vec![4, 3], vec![1, 4], vec![4, 6], vec![1, 7]]
            ),
            4
        );
    }
}
