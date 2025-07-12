pub struct Solution {}

impl Solution {
    pub fn recurse(
        state: i32,
        l: usize,
        r: usize,
        round: i32,
        first_player: usize,
        second_player: usize,
        states: &mut Vec<i32>,
        result: &mut Vec<i32>,
    ) {
        let mut l = l;
        while l > r && state & 1 << l != 0 {
            l -= 1
        }
        let mut r = r;
        while l > r && state & 1 << r != 0 {
            r += 1
        }
        if l <= r {
            states.push(state);
            return;
        }
        if r == first_player && l == second_player {
            result[0] = result[0].min(round);
            result[1] = result[1].max(round);
            return;
        }
        if l != second_player && l != first_player {
            Self::recurse(
                state | 1 << l,
                l - 1,
                r + 1,
                round,
                first_player,
                second_player,
                states,
                result,
            );
        }
        if r != second_player && r != first_player {
            Self::recurse(
                state | 1 << r,
                l - 1,
                r + 1,
                round,
                first_player,
                second_player,
                states,
                result,
            );
        }
    }
    pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
        let mut states = vec![0];
        let mut result = vec![std::i32::MAX, std::i32::MIN];
        let mut round = 0;

        while !states.is_empty() {
            round += 1;
            let mut new_states = Vec::new();
            for state in states {
                Self::recurse(
                    state,
                    n as usize - 1,
                    0,
                    round,
                    first_player as usize - 1,
                    second_player as usize - 1,
                    &mut new_states,
                    &mut result,
                );
            }
            states = new_states;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1900() {
        assert_eq!(Solution::earliest_and_latest(11, 2, 4), vec![3, 4]);
        assert_eq!(Solution::earliest_and_latest(5, 1, 5), vec![1, 1]);
    }
}
