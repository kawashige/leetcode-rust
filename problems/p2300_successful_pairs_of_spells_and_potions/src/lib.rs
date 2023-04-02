pub struct Solution {}

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut potions = potions;
        potions.sort_unstable_by(|a, b| b.cmp(&a));

        let mut result = vec![0; spells.len()];

        let mut spells = spells.into_iter().zip(0..).collect::<Vec<_>>();
        spells.sort_unstable();

        let mut i = 0;
        for (spell, j) in spells {
            while i < potions.len() && success <= potions[i] as i64 * spell as i64 {
                i += 1;
            }
            result[j] = i as i32;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2300() {
        assert_eq!(
            Solution::successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7),
            vec![4, 0, 3]
        );
        assert_eq!(
            Solution::successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16),
            vec![2, 0, 2]
        );
    }
}
