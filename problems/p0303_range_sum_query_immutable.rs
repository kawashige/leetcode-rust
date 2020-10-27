struct NumArray {
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.nums[i as usize..=j as usize].iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0303() {
        let obj = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(1, obj.sum_range(0, 2));
        assert_eq!(-1, obj.sum_range(2, 5));
        assert_eq!(-3, obj.sum_range(0, 5));
    }
}
