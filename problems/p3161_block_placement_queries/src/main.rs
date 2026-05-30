pub struct Solution {}

use std::collections::BTreeSet;

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mx = 50000;
        let mut st = BTreeSet::new();
        st.insert(0);
        st.insert(mx);

        for q in &queries {
            if q[0] == 1 {
                st.insert(q[1]);
            }
        }

        let mut bt = vec![0; (mx + 1) as usize];

        fn update(bt: &mut Vec<i32>, x: i32, v: i32) {
            let mut x = x as usize;
            while x < bt.len() {
                bt[x] = bt[x].max(v);
                x += x & (!x + 1);
            }
        }

        fn query(bt: &Vec<i32>, x: i32) -> i32 {
            let mut x = x as usize;
            let mut res = 0;
            while x > 0 {
                res = res.max(bt[x]);
                x -= x & (!x + 1);
            }
            res
        }

        let mut pre = 0;
        for &x in st.iter() {
            if x == 0 {
                continue;
            }
            update(&mut bt, x, x - pre);
            pre = x;
        }

        let mut ans = Vec::new();
        for i in (0..queries.len()).rev() {
            let q = &queries[i];
            if q[0] == 2 {
                let x = q[1];
                let sz = q[2];
                let pre_val = st.range(..=x).next_back().copied().unwrap_or(0);
                let max_space = query(&bt, pre_val).max(x - pre_val);
                ans.push(max_space >= sz);
            } else {
                let x = q[1];
                let pre_val = st.range(..x).next_back().copied().unwrap_or(0);
                let nxt = st.range((x + 1)..).next().copied().unwrap_or(mx);
                update(&mut bt, nxt, nxt - pre_val);
                st.remove(&x);
            }
        }

        ans.reverse();
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3161() {
        assert_eq!(
            Solution::get_results(vec![
                vec![1, 2],
                vec![2, 3, 3],
                vec![2, 3, 1],
                vec![2, 2, 2]
            ]),
            vec![false, true, true]
        );
        assert_eq!(
            Solution::get_results(vec![
                vec![1, 7],
                vec![2, 7, 6],
                vec![1, 2],
                vec![2, 7, 5],
                vec![2, 7, 6]
            ]),
            vec![true, true, false]
        );
    }
}

fn main() {}
