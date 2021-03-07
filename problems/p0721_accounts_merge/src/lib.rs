use std::collections::{BTreeSet, HashMap};
pub struct Solution {}

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<BTreeSet<String>>> = HashMap::new();

        for mut account in accounts {
            let name = account.remove(0);
            let mut addresses = account.into_iter().collect::<BTreeSet<String>>();
            if let Some(sets) = map.get_mut(&name) {
                let mut next = vec![];
                for mut set in sets.clone() {
                    if set.is_disjoint(&addresses) {
                        next.push(set);
                    } else {
                        addresses.append(&mut set);
                    }
                }
                next.push(addresses);
                *sets = next;
            } else {
                map.insert(name, vec![addresses]);
            }
        }

        let mut result = Vec::new();
        for (k, addresses) in map.into_iter() {
            for address in addresses {
                result.push(
                    std::iter::once(k.clone())
                        .chain(address.into_iter())
                        .collect(),
                );
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0721() {
        assert_eq!(
            Solution::accounts_merge(vec![
                vec![
                    "John".to_string(),
                    "johnsmith@mail.com".to_string(),
                    "john00@mail.com".to_string()
                ],
                vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
                vec![
                    "John".to_string(),
                    "johnsmith@mail.com".to_string(),
                    "john_newyork@mail.com".to_string()
                ],
                vec!["Mary".to_string(), "mary@mail.com".to_string()]
            ]),
            vec![
                vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
                vec![
                    "John".to_string(),
                    "john00@mail.com".to_string(),
                    "john_newyork@mail.com".to_string(),
                    "johnsmith@mail.com".to_string()
                ],
                vec!["Mary".to_string(), "mary@mail.com".to_string()]
            ]
        )
    }
}
