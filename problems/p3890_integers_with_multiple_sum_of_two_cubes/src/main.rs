use std::collections::BTreeMap;

pub struct Solution {}

impl Solution {
    pub fn find_good_integers(n: i32) -> Vec<i32> {
        let mut triples = Vec::new();
        for i in 1..=n {
            let x = i * i * i;
            if n < x {
                break;
            }
            triples.push(x);
        }

        let mut map = BTreeMap::new();
        for i in 0..triples.len() {
            for j in 0..i {
                if triples[i] + triples[j] <= n {
                    *map.entry(triples[i] + triples[j]).or_insert(0) += 1;
                }
            }
        }

        map.into_iter()
            .filter_map(|(k, v)| if 1 < v { Some(k) } else { None })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3890() {
        assert_eq!(Solution::find_good_integers(4104), vec![1729, 4104]);
        assert_eq!(Solution::find_good_integers(578), vec![]);
    }
}

fn main() {}
