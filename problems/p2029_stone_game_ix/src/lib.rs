pub struct Solution {}

impl Solution {
    pub fn recurse(current: usize, mods: &mut [usize], is_alice: bool) -> bool {
        if mods.iter().all(|i| &0 == i) {
            return false;
        }
        if current == 1 {
            if 0 < mods[1] {
                mods[1] -= 1;
                Self::recurse(2, mods, !is_alice)
            } else if 0 < mods[0] {
                mods[0] -= 1;
                Self::recurse(1, mods, !is_alice)
            } else {
                !is_alice
            }
        } else {
            if 0 < mods[2] {
                mods[2] -= 1;
                Self::recurse(1, mods, !is_alice)
            } else if 0 < mods[0] {
                mods[0] -= 1;
                Self::recurse(2, mods, !is_alice)
            } else {
                !is_alice
            }
        }
    }

    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        if stones.len() == 1 {
            return false;
        }

        let mut mods = stones.into_iter().fold([0; 3], |mut mods, stone| {
            mods[stone as usize % 3] += 1;
            mods
        });

        for i in 1..mods.len() {
            if mods[i] == 0 {
                continue;
            }
            mods[i] -= 1;
            if Self::recurse(i, &mut mods.clone(), false) {
                return true;
            }
            mods[i] += 1;
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2029() {
        assert!(!Solution::stone_game_ix(vec![
            15, 20, 10, 13, 14, 15, 5, 2, 3
        ]));
        assert!(!Solution::stone_game_ix(vec![2, 2, 3]));
        assert!(Solution::stone_game_ix(vec![
            20, 3, 20, 17, 2, 12, 15, 17, 4
        ]));
        assert!(Solution::stone_game_ix(vec![1, 11, 12, 17, 6]));
        assert!(Solution::stone_game_ix(vec![2, 1]));
        assert!(!Solution::stone_game_ix(vec![2]));
        assert!(!Solution::stone_game_ix(vec![5, 1, 2, 4, 3]));
    }
}
