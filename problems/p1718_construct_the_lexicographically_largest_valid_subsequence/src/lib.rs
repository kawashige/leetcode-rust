pub struct Solution {}

impl Solution {
    pub fn recurse(i: usize, sequence: &mut Vec<i32>, used: &mut Vec<bool>) -> bool {
        if i == sequence.len() {
            return true;
        }

        if sequence[i] != 0 {
            return Self::recurse(i + 1, sequence, used);
        }

        for j in (1..used.len()).rev() {
            if used[j] || (1 < j && (sequence.len() <= j + i || sequence[j + i] != 0)) {
                continue;
            }

            sequence[i] = j as i32;
            if 1 < j {
                sequence[j + i] = j as i32;
            }
            used[j] = true;
            if Self::recurse(i + 1, sequence, used) {
                return true;
            }
            sequence[i] = 0;
            if 1 < j {
                sequence[j + i] = 0;
            }
            used[j] = false;
        }

        false
    }

    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let mut sequence = vec![0; n as usize * 2 - 1];
        let mut used = vec![false; n as usize + 1];

        Self::recurse(0, &mut sequence, &mut used);

        sequence
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1718() {
        assert_eq!(
            Solution::construct_distanced_sequence(4),
            vec![4, 2, 3, 2, 4, 3, 1]
        );
        assert_eq!(
            Solution::construct_distanced_sequence(3),
            vec![3, 1, 2, 3, 2]
        );
        assert_eq!(
            Solution::construct_distanced_sequence(5),
            vec![5, 3, 1, 4, 3, 5, 2, 4, 2]
        );
    }
}
