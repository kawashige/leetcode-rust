use std::collections::HashMap;

struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    nums2_count: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let nums2_count = nums2.iter().fold(HashMap::new(), |mut map, num| {
            *map.entry(*num).or_insert(0) += 1;
            map
        });
        Self {
            nums1,
            nums2,
            nums2_count,
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        *self
            .nums2_count
            .get_mut(&self.nums2[index as usize])
            .unwrap() -= 1;
        self.nums2[index as usize] += val;
        *self
            .nums2_count
            .entry(self.nums2[index as usize])
            .or_insert(0) += 1;
    }

    fn count(&self, tot: i32) -> i32 {
        self.nums1
            .iter()
            .map(|num1| {
                if &tot <= num1 {
                    0
                } else {
                    *self.nums2_count.get(&(tot - num1)).unwrap_or(&0)
                }
            })
            .sum::<i32>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1865() {
        let mut obj = FindSumPairs::new(vec![1, 1, 2, 2, 2, 3], vec![1, 4, 5, 2, 5, 4]);
        assert_eq!(obj.count(7), 8);
        obj.add(3, 2);
        assert_eq!(obj.count(8), 2);
        assert_eq!(obj.count(4), 1);
        obj.add(0, 1);
        obj.add(1, 1);
        assert_eq!(obj.count(7), 11);
    }
}
