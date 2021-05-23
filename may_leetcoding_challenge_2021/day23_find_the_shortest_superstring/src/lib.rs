pub struct Solution {}

impl Solution {
    pub fn recurse(
        dp: &mut Vec<Vec<i32>>,
        cost: &Vec<Vec<i32>>,
        pre: &mut Vec<Vec<usize>>,
        bit: usize,
        v: usize,
    ) -> i32 {
        if dp[bit][v] != -1 {
            return dp[bit][v];
        }

        if bit == 1 << v {
            return cost[v][v];
        }

        let mut result = 10_000;
        let prev = bit & !(1 << v);

        for u in 0..dp[0].len() {
            if prev & 1 << u == 0 {
                continue;
            }

            if result > Self::recurse(dp, cost, pre, prev, u) + cost[u][v] {
                result = Self::recurse(dp, cost, pre, prev, u) + cost[u][v];
                pre[bit][v] = u;
            }
        }

        dp[bit][v] = result;
        result
    }

    pub fn shortest_superstring(words: Vec<String>) -> String {
        let mut cost = vec![vec![0; words.len()]; words.len()];
        for i in 0..words.len() {
            for j in 0..words.len() {
                if i == j {
                    cost[i][j] = words[i].len() as i32;
                } else {
                    let bytes_i = words[i].as_bytes();
                    let bytes_j = words[j].as_bytes();
                    cost[i][j] = (bytes_j.len()
                        - (1..std::cmp::min(bytes_i.len(), bytes_j.len()))
                            .rev()
                            .find(|k| bytes_i[(bytes_i.len() - k)..] == bytes_j[..*k])
                            .unwrap_or(0)) as i32;
                }
            }
        }

        let mut dp = vec![vec![-1; words.len()]; 1 << words.len()];
        let mut pre = vec![vec![0; words.len()]; 1 << words.len()];

        let mut result = 10_000;
        let mut id = 0;
        for v in 0..words.len() {
            if result > Self::recurse(&mut dp, &cost, &mut pre, (1 << words.len()) - 1, v) {
                result = Self::recurse(&mut dp, &cost, &mut pre, (1 << words.len()) - 1, v);
                id = v;
            }
        }

        let mut bit = (1 << words.len()) - 1;
        let mut order = Vec::with_capacity(words.len());

        while bit > 0 {
            order.push(id);
            let tmp = id;
            id = pre[bit][id];
            bit &= !(1 << tmp);
        }

        order = order.into_iter().rev().collect();
        let mut s = words[order[0]].clone();
        for i in 1..order.len() {
            s +=
                &words[order[i]][(words[order[i]].len() - cost[order[i - 1]][order[i]] as usize)..];
        }
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day23() {
        assert_eq!(
            Solution::shortest_superstring(vec![
                "alex".to_string(),
                "loves".to_string(),
                "leetcode".to_string()
            ]),
            "leetcodelovesalex".to_string()
        );

        assert_eq!(
            Solution::shortest_superstring(vec![
                "catg".to_string(),
                "ctaagt".to_string(),
                "gcta".to_string(),
                "ttca".to_string(),
                "atgcatc".to_string()
            ]),
            "gctaagttcatgcatc".to_string()
        );
    }
}
