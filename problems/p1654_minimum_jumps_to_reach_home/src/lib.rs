use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        let a = a as usize;
        let b = b as usize;
        let x = x as usize;

        let mut seen = vec![0; x as usize + a * b];
        for f in forbidden {
            if (f as usize) < seen.len() {
                seen[f as usize] = 2;
            }
        }

        let mut queue = VecDeque::new();
        queue.push_back((0, 0, false));

        while let Some((count, pos, is_prev_back)) = queue.pop_front() {
            if pos == x {
                return count;
            }
            if seen[pos] == 2 || (seen[pos] == 1 && is_prev_back) {
                continue;
            }
            seen[pos] = if is_prev_back { 1 } else { 2 };

            if pos + a < seen.len() && seen[pos + a] != 2 {
                queue.push_back((count + 1, pos + a, false));
            }
            if b < pos && !is_prev_back && seen[pos - b] == 0 {
                queue.push_back((count + 1, pos - b, true));
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1654() {
        assert_eq!(
            Solution::minimum_jumps(
                vec![
                    162, 118, 178, 152, 167, 100, 40, 74, 199, 186, 26, 73, 200, 127, 30, 124, 193,
                    84, 184, 36, 103, 149, 153, 9, 54, 154, 133, 95, 45, 198, 79, 157, 64, 122, 59,
                    71, 48, 177, 82, 35, 14, 176, 16, 108, 111, 6, 168, 31, 134, 164, 136, 72, 98
                ],
                29,
                98,
                80,
            ),
            121
        );
        assert_eq!(Solution::minimum_jumps(vec![14, 4, 18, 1, 15], 3, 15, 9), 3);
        assert_eq!(
            Solution::minimum_jumps(vec![8, 3, 16, 6, 12, 20], 15, 13, 11),
            -1
        );
        assert_eq!(
            Solution::minimum_jumps(vec![1, 6, 2, 14, 5, 17, 4], 16, 9, 7),
            2
        );
    }
}
