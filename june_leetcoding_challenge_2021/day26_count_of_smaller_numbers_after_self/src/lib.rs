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
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        const BASE: usize = 10_001;
        let mut result = Vec::with_capacity(nums.len());

        let mut bit = Bit::new(2 * BASE + 1);
        let mut min = 2 * BASE + 1;
        for i in (0..nums.len()).rev() {
            let x = (nums[i] + BASE as i32) as usize;
            if min < x && x > 0 {
                result.push(bit.sum(x - 1) as i32);
            } else {
                min = x;
                result.push(0);
            }
            bit.add(x, 1);
        }

        result.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day26() {
        assert_eq!(Solution::count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
        assert_eq!(Solution::count_smaller(vec![-1]), vec![0]);
        assert_eq!(Solution::count_smaller(vec![-1, -1]), vec![0, 0]);
    }
}
