pub struct Solution {}

impl Solution {
    pub fn recurse(
        i: usize,
        start: i32,
        number: i32,
        edges: &[i32],
        seen: &mut Vec<(i32, i32)>,
    ) -> i32 {
        if seen[i].0 == start {
            return number - seen[i].1;
        }
        if seen[i].0 != -1 {
            return -1;
        }
        seen[i] = (start, number);

        if edges[i] == -1 {
            -1
        } else {
            Self::recurse(edges[i] as usize, start, number + 1, edges, seen)
        }
    }

    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let mut seen = vec![(-1, -1); edges.len()];
        let mut result = -1;

        for i in 0..edges.len() {
            if seen[i].0 != -1 {
                continue;
            }
            result = result.max(Self::recurse(i, i as i32, 0, &edges, &mut seen));
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2360() {
        assert_eq!(Solution::longest_cycle(vec![3, 3, 4, 2, 3]), 3);
        assert_eq!(Solution::longest_cycle(vec![2, -1, 3, 1]), -1);
    }
}
