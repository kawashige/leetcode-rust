pub struct Solution {}

struct Bit {
    n: usize,
    pub data: Vec<usize>,
}

impl Bit {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![0; n + 1],
        }
    }

    pub fn sum(&self, i: usize) -> usize {
        let mut r = 0;
        let mut i = i as i32;
        while i > 0 {
            r += self.data[i as usize];
            i -= i & -i;
        }
        r
    }

    pub fn add(&mut self, i: usize, x: usize) {
        let mut i = i as i32;
        while i as usize <= self.n {
            self.data[i as usize] += x;
            i += i & -i;
        }
    }
}
impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut nums2_indices = vec![0; nums2.len()];
        for i in 0..nums2.len() {
            nums2_indices[nums2[i] as usize] = i;
        }
        let mut bit = Bit::new(nums1.len());
        let mut result = 0;

        for i1 in 0..nums1.len() {
            let i2 = nums2_indices[nums1[i1] as usize];
            let before = bit.sum(i2 + 1);
            let after = nums1.len() - i1 - 1 - (i2 - before);
            result += before as i64 * after as i64;
            bit.add(i2 + 1, 1);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2179() {
        assert_eq!(
            Solution::good_triplets(vec![2, 0, 1, 3], vec![0, 1, 2, 3]),
            1
        );
        assert_eq!(
            Solution::good_triplets(vec![4, 0, 1, 3, 2], vec![4, 1, 0, 2, 3]),
            4
        );
    }
}
