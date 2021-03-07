use std::collections::HashMap;
pub struct Solution {}

pub struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn root(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }

        let parent = self.root(self.parent[i]);
        self.parent[i] = parent;
        parent
    }

    pub fn unite(&mut self, i: usize, j: usize) {
        let mut parent_i = self.root(i);
        let mut parent_j = self.root(j);

        if parent_i == parent_j {
            return;
        }

        if self.size(parent_i) > self.size(parent_j) {
            std::mem::swap(&mut parent_i, &mut parent_j);
        }

        self.parent[parent_j] = parent_i;
        self.size[parent_i] += self.size[parent_j];
    }

    fn size(&mut self, i: usize) -> usize {
        let root = self.root(i);
        self.size[root]
    }
}

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut disjoint_set = DisjointSet::new(10001);

        let mut email_to_name = HashMap::new();
        let mut email_to_id = HashMap::new();

        let mut id = 0;
        for mut account in accounts {
            let name = account.remove(0);
            let first_email = account[0].clone();
            for email in account {
                email_to_name.entry(email.clone()).or_insert(name.clone());
                if email_to_id.get(&email).is_none() {
                    email_to_id.insert(email.clone(), id);
                    id += 1;
                }
                disjoint_set.unite(email_to_id[&first_email], email_to_id[&email])
            }
        }

        let mut result = HashMap::new();
        for email in email_to_name.keys() {
            (*result
                .entry(disjoint_set.root(email_to_id[email]))
                .or_insert(Vec::new()))
            .push(email.clone());
        }

        result
            .values()
            .map(|v| {
                let mut v: Vec<String> = v.iter().cloned().collect();
                v.sort_unstable();
                let mut r = vec![email_to_name[&v[0]].clone()];
                r.append(&mut v);
                r
            })
            .collect()
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
                vec!["Mary".to_string(), "mary@mail.com".to_string()],
                vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
                vec![
                    "John".to_string(),
                    "john00@mail.com".to_string(),
                    "john_newyork@mail.com".to_string(),
                    "johnsmith@mail.com".to_string()
                ],
            ]
        )
    }
}
