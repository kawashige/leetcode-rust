struct NumArray {
    nums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    fn update(&mut self, i: i32, val: i32) {
        self.nums[i as usize] = val;
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.nums[i as usize..=j as usize].iter().sum::<i32>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0307() {
        let mut obj = NumArray::new(vec![1, 2, 3]);
        assert_eq!(6, obj.sum_range(0, 2));
        obj.update(1, 10);
        assert_eq!(14, obj.sum_range(0, 2));
    }
}
