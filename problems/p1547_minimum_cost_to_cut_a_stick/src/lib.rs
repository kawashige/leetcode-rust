use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn recurse(
        stick: &[i32],
        l: usize,
        r: usize,
        memo: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if l + 1 == r {
            return 0;
        }
        if let Some(result) = memo.get(&(l, r)) {
            return *result;
        }

        let mut min_cost = std::i32::MAX;

        for i in l + 1..r {
            min_cost =
                min_cost.min(Self::recurse(stick, l, i, memo) + Self::recurse(stick, i, r, memo))
        }

        let result = min_cost + stick[r] - stick[l];
        memo.insert((l, r), result);
        result
    }

    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        let mut stick = cuts;
        stick.push(0);
        stick.push(n);
        stick.sort_unstable();

        Self::recurse(&stick, 0, stick.len() - 1, &mut HashMap::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1547() {
        assert_eq!(Solution::min_cost(7, vec![1, 3, 4, 5]), 16);
        assert_eq!(Solution::min_cost(9, vec![5, 6, 1, 4, 2]), 22);
    }
}
