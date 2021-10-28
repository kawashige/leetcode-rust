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

        if self.size(parent_i) < self.size(parent_j) {
            std::mem::swap(&mut parent_i, &mut parent_j);
        }

        self.parent[parent_j] = parent_i;
        self.size[parent_i] += self.size[parent_j];
    }

    pub fn size(&mut self, i: usize) -> usize {
        let root = self.root(i);
        self.size[root]
    }
}

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let (equal, not_equal): (Vec<(bool, (usize, usize))>, Vec<(bool, (usize, usize))>) =
            equations
                .into_iter()
                .map(|e| {
                    let chars = e.as_bytes();
                    let d1 = (chars[0] - 0x61) as usize;
                    let d2 = (chars[3] - 0x61) as usize;
                    let eq = chars[1] != b'!';
                    (eq, (d1, d2))
                })
                .partition(|(x, _)| *x);

        let mut ds = DisjointSet::new(26);
        for (_, (d1, d2)) in equal {
            ds.unite(d1, d2);
        }

        not_equal
            .into_iter()
            .all(|(_, (d1, d2))| ds.root(d1) != ds.root(d2))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0990() {
        assert!(Solution::equations_possible(vec![
            "c==c".to_string(),
            "f!=a".to_string(),
            "f==b".to_string(),
            "b==c".to_string()
        ]));
        assert!(!Solution::equations_possible(vec!["a!=a".to_string()]));
        assert!(!Solution::equations_possible(vec![
            "a==b".to_string(),
            "b!=a".to_string()
        ]));
        assert!(Solution::equations_possible(vec![
            "b==a".to_string(),
            "a==b".to_string()
        ]));
        assert!(Solution::equations_possible(vec![
            "a==b".to_string(),
            "b==c".to_string(),
            "a==c".to_string()
        ]));
        assert!(!Solution::equations_possible(vec![
            "a==b".to_string(),
            "b!=c".to_string(),
            "c==a".to_string()
        ]));
        assert!(Solution::equations_possible(vec![
            "c==c".to_string(),
            "b==d".to_string(),
            "x!=z".to_string()
        ]));
    }
}
