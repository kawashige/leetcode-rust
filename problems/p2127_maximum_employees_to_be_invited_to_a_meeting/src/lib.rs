pub struct Solution {}

impl Solution {
    pub fn recurse(i: usize, c: i32, favorite: &[i32], seen: &mut Vec<i32>) -> i32 {
        if seen[i] != -1 {
            return if c - 2 == seen[i] { c - 1 } else { c - seen[i] };
        }
        seen[i] = c;
        Self::recurse(favorite[i] as usize, c + 1, favorite, seen)
    }

    pub fn recurse2(i: usize, list: &Vec<Vec<usize>>, seen: &mut Vec<bool>) -> i32 {
        let mut result = 0;
        for n in &list[i] {
            if seen[*n] {
                continue;
            }
            seen[*n] = true;
            result = result.max(Self::recurse2(*n, list, seen));
        }
        result + 1
    }

    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let mut in_count = vec![0; favorite.len()];
        let mut list = vec![vec![]; favorite.len()];
        for i in 0..favorite.len() {
            in_count[favorite[i] as usize] += 1;
            list[favorite[i] as usize].push(i);
        }
        let mut stack = Vec::new();
        for i in 0..in_count.len() {
            if in_count[i] == 0 {
                stack.push(i);
            }
        }
        while let Some(i) = stack.pop() {
            in_count[favorite[i] as usize] -= 1;
            if in_count[favorite[i] as usize] == 0 {
                stack.push(favorite[i] as usize);
            }
        }

        let mut seen = vec![-1; in_count.len()];
        let mut result = 0;
        for i in 0..in_count.len() {
            if in_count[i] == 0 || seen[i] != -1 {
                continue;
            }
            result = result.max(Self::recurse(i, 1, &favorite, &mut seen));
        }

        let mut seen = vec![false; favorite.len()];
        let mut result2 = 0;
        for i in 0..favorite.len() {
            if seen[i] {
                continue;
            }
            if i == favorite[favorite[i] as usize] as usize {
                seen[i] = true;
                seen[favorite[i] as usize] = true;
                result2 += Self::recurse2(i, &list, &mut seen)
                    + Self::recurse2(favorite[i] as usize, &list, &mut seen);
            }
        }

        result.max(result2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2127() {
        assert_eq!(
            Solution::maximum_invitations(vec![
                23, 14, 17, 5, 19, 2, 0, 18, 15, 12, 2, 8, 21, 3, 3, 1, 6, 5, 11, 17, 3, 7, 14, 13
            ]),
            3
        );
        assert_eq!(
            Solution::maximum_invitations(vec![1, 0, 0, 2, 1, 4, 7, 8, 9, 6, 7, 10, 8]),
            6
        );
        assert_eq!(Solution::maximum_invitations(vec![2, 2, 1, 2]), 3);
        assert_eq!(Solution::maximum_invitations(vec![1, 2, 0]), 3);
        assert_eq!(Solution::maximum_invitations(vec![3, 0, 1, 4, 1]), 4);
    }
}
