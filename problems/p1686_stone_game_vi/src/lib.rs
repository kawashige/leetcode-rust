pub struct Solution {}

impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let mut sum_values = alice_values
            .into_iter()
            .zip(bob_values.into_iter())
            .map(|(a, b)| (a + b, a, b))
            .collect::<Vec<_>>();
        sum_values.sort_unstable_by(|a, b| b.cmp(&a));
        let (alice, bob) =
            sum_values
                .into_iter()
                .enumerate()
                .fold((0, 0), |(alice, bob), (i, (_, a, b))| {
                    if i % 2 == 0 {
                        (alice + a, bob)
                    } else {
                        (alice, bob + b)
                    }
                });

        if alice == bob {
            0
        } else if alice < bob {
            -1
        } else {
            1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1686() {
        assert_eq!(Solution::stone_game_vi(vec![1, 3], vec![2, 1]), 1);
        assert_eq!(Solution::stone_game_vi(vec![1, 2], vec![3, 1]), 0);
        assert_eq!(Solution::stone_game_vi(vec![2, 4, 3], vec![1, 6, 7]), -1);
    }
}
