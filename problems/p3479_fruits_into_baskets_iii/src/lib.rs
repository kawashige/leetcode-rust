pub struct Solution {}
struct SegmentTree {
    seg_node: Vec<i32>,
    baskets: Vec<i32>,
}

impl SegmentTree {
    fn new(baskets: Vec<i32>) -> Self {
        let n = baskets.len();
        let mut tree = SegmentTree {
            seg_node: vec![0; 4 * n + 7],
            baskets,
        };
        tree.build(1, 0, n - 1);
        tree
    }

    fn build(&mut self, p: usize, l: usize, r: usize) {
        if l == r {
            self.seg_node[p] = self.baskets[l];
            return;
        }
        let mid = (l + r) / 2;
        self.build(p * 2, l, mid);
        self.build(p * 2 + 1, mid + 1, r);
        self.seg_node[p] = self.seg_node[p * 2].max(self.seg_node[p * 2 + 1]);
    }

    fn query(&self, p: usize, l: usize, r: usize, ql: usize, qr: usize) -> i32 {
        if ql > r || qr < l {
            return i32::MIN;
        }
        if ql <= l && r <= qr {
            return self.seg_node[p];
        }
        let mid = (l + r) / 2;
        self.query(p * 2, l, mid, ql, qr)
            .max(self.query(p * 2 + 1, mid + 1, r, ql, qr))
    }

    fn update(&mut self, p: usize, l: usize, r: usize, pos: usize, val: i32) {
        if l == r {
            self.seg_node[p] = val;
            return;
        }
        let mid = (l + r) / 2;
        if pos <= mid {
            self.update(p * 2, l, mid, pos, val);
        } else {
            self.update(p * 2 + 1, mid + 1, r, pos, val);
        }
        self.seg_node[p] = self.seg_node[p * 2].max(self.seg_node[p * 2 + 1]);
    }
}

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        if baskets.is_empty() {
            return fruits.len() as i32;
        }
        let m = baskets.len();

        let mut tree = SegmentTree::new(baskets);
        let mut count = 0;

        for fruit in fruits {
            let (mut l, mut r, mut res) = (0 as i32, (m - 1) as i32, None);
            while l <= r {
                let mid = (l + r) / 2;
                if tree.query(1, 0, m - 1, 0, mid as usize) >= fruit {
                    res = Some(mid);
                    r = mid - 1;
                } else {
                    l = mid + 1;
                }
            }

            if let Some(pos) = res {
                if tree.baskets[pos as usize] >= fruit {
                    tree.update(1, 0, m - 1, pos as usize, i32::MIN);
                    continue;
                }
            }
            count += 1;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3479() {
        assert_eq!(Solution::num_of_unplaced_fruits(vec![52], vec![83]), 0);
        assert_eq!(
            Solution::num_of_unplaced_fruits(vec![4, 2, 5], vec![3, 5, 4]),
            1
        );
        assert_eq!(
            Solution::num_of_unplaced_fruits(vec![3, 6, 1], vec![6, 4, 7]),
            0
        );
    }
}
