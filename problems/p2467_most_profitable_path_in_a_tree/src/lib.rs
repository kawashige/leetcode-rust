pub struct Solution {}

impl Solution {
    pub fn recurse(
        i: usize,
        list: &Vec<Vec<usize>>,
        path: &mut Vec<usize>,
        seen: &mut Vec<bool>,
    ) -> bool {
        if i == 0 {
            return true;
        }
        seen[i] = true;

        for next in &list[i] {
            if seen[*next] {
                continue;
            }
            path.push(*next);
            if Self::recurse(*next, list, path, seen) {
                return true;
            }
            path.pop();
        }

        false
    }

    pub fn recurse2(
        i: usize,
        t: usize,
        current_amount: i32,
        list: &Vec<Vec<usize>>,
        amount: &[i32],
        bob_time: &[usize],
        seen: &mut Vec<bool>,
        max_amount: &mut i32,
    ) {
        if seen[i] {
            return;
        }
        seen[i] = true;

        let mut current_amount = current_amount;
        if t < bob_time[i] {
            current_amount += amount[i];
        } else if t == bob_time[i] {
            current_amount += amount[i] / 2;
        }

        if list[i].len() == 1 && i != 0 {
            *max_amount = std::cmp::max(*max_amount, current_amount);
        }

        for next in &list[i] {
            if seen[*next] {
                continue;
            }
            Self::recurse2(
                *next,
                t + 1,
                current_amount,
                list,
                amount,
                bob_time,
                seen,
                max_amount,
            );
        }
    }

    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let mut list = vec![vec![]; edges.len() + 1];
        for edge in edges {
            list[edge[0] as usize].push(edge[1] as usize);
            list[edge[1] as usize].push(edge[0] as usize);
        }

        let mut path = vec![bob as usize];
        let mut seen = vec![false; list.len()];

        Self::recurse(bob as usize, &list, &mut path, &mut seen);

        let mut bob_time = vec![list.len(); list.len()];
        for i in 0..path.len() {
            bob_time[path[i]] = i;
        }

        println!("bob_time: {:?}", bob_time);

        let mut max_amount = std::i32::MIN;
        let mut seen = vec![false; list.len()];
        Self::recurse2(
            0,
            0,
            0,
            &list,
            &amount,
            &bob_time,
            &mut seen,
            &mut max_amount,
        );
        max_amount
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2467() {
        assert_eq!(
            Solution::most_profitable_path(
                vec![vec![0, 1], vec![1, 2], vec![2, 3]],
                3,
                vec![-5644, -6018, 1188, -8502]
            ),
            -11662
        );
        assert_eq!(
            Solution::most_profitable_path(
                vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]],
                3,
                vec![-2, 4, 2, -4, 6]
            ),
            6
        );
        assert_eq!(
            Solution::most_profitable_path(vec![vec![0, 1]], 1, vec![-7280, 2350]),
            -7280
        );
    }
}
