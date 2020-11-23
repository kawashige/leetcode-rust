use rand::{thread_rng, Rng};
pub struct Solution {
    org: Vec<i32>,
    array: Vec<i32>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            org: nums.clone(),
            array: nums,
        }
    }

    /** Resets the array to its original configuration and return it. */
    fn reset(&mut self) -> Vec<i32> {
        self.array = self.org.clone();
        self.org.clone()
    }

    /** Returns a random shuffling of the array. */
    fn shuffle(&mut self) -> Vec<i32> {
        let mut tmp = self.array.clone();
        let mut rng = thread_rng();
        for i in 0..self.array.len() {
            let pos = rng.gen_range(0, tmp.len());
            self.array[i] = tmp.remove(pos);
        }
        self.array.clone()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0384() {
        let mut obj = Solution::new(vec![1, 2, 3]);
        assert!(obj.shuffle().into_iter().all(|s| [1, 2, 3].contains(&s)));
        assert_eq!(vec![1, 2, 3], obj.reset());
        assert!(obj.shuffle().into_iter().all(|s| [1, 2, 3].contains(&s)));
    }
}
