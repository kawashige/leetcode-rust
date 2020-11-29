use rand::{thread_rng, Rng};

pub struct Solution {
    nums: Vec<i32>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    fn pick(&self, target: i32) -> i32 {
        let mut rng = thread_rng();
        let mut scope = 1.0;
        let mut result = 0;
        for i in 0..self.nums.len() {
            if self.nums[i] == target {
                if rng.gen::<f64>() < 1.0 / scope {
                    result = i;
                }
                scope += 1.0;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0398() {
        let obj = Solution::new(vec![1, 2, 3, 3, 3]);
        assert!([2, 3, 4].contains(&obj.pick(3)));
        assert_eq!(0, obj.pick(1));
        assert_eq!(1, obj.pick(2));
    }
}
