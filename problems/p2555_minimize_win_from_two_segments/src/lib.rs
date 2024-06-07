pub struct Solution {}

impl Solution {
    pub fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
        if prize_positions.len() < 3 {
            return prize_positions.len() as i32;
        }

        let mut right_max = vec![1; prize_positions.len()];
        let mut r = prize_positions.len() - 1;
        for l in (0..prize_positions.len() - 1).rev() {
            while k < prize_positions[r] - prize_positions[l] {
                r -= 1;
            }
            right_max[l] = right_max[l + 1].max(r - l + 1);
        }

        let mut result = 1 + right_max[1];

        let mut left_max = vec![1; prize_positions.len()];
        let mut l = 0;
        for r in 1..prize_positions.len() - 1 {
            while k < prize_positions[r] - prize_positions[l] {
                l += 1;
            }
            left_max[r] = left_max[r - 1].max(r - l + 1);
            result = result.max(left_max[r] + right_max[r + 1])
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2555() {
        assert_eq!(Solution::maximize_win(vec![1, 1, 2, 2, 3, 3, 5], 2), 7);
        assert_eq!(Solution::maximize_win(vec![1, 2, 3, 4], 0), 2);
    }
}
