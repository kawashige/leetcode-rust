struct RangeFreqQuery {
    value_indices: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        let mut value_indices = vec![vec![]; 10_001];
        for i in 0..arr.len() {
            value_indices[arr[i] as usize].push(i as i32);
        }

        Self { value_indices }
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        (match (
            self.value_indices[value as usize].binary_search(&left),
            self.value_indices[value as usize].binary_search(&right),
        ) {
            (Ok(l), Ok(r)) => r - l + 1,
            (Err(l), Ok(r)) => r - l + 1,
            (Ok(l), Err(r)) => r - l,
            (Err(l), Err(r)) => r - l,
        }) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2080() {
        let mut obj = RangeFreqQuery::new(vec![12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]);
        assert_eq!(obj.query(1, 2, 4), 1);
        assert_eq!(obj.query(0, 11, 33), 2);
    }
}
