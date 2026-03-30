struct SnapshotArray {
    values: Vec<Vec<(i32, i32)>>,
    current_id: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {
    fn new(length: i32) -> Self {
        Self {
            values: vec![vec![]; length as usize],
            current_id: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.values[index as usize].push((self.current_id * 2, val));
    }

    fn snap(&mut self) -> i32 {
        self.current_id += 1;
        self.current_id - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        match self.values[index as usize].binary_search_by_key(&(snap_id * 2 + 1), |(id, _)| *id) {
            Err(i) if i == 0 => 0,
            Err(i) => self.values[index as usize][i - 1].1,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1146() {
        let mut obj = SnapshotArray::new(3);
        obj.set(0, 5);
        assert_eq!(obj.snap(), 0);
        obj.set(0, 6);
        assert_eq!(obj.get(0, 0), 5);

        let mut obj = SnapshotArray::new(1);
        obj.set(0, 4);
        obj.set(0, 16);
        obj.set(0, 13);
        assert_eq!(obj.snap(), 0);
        obj.set(0, 13);
        assert_eq!(obj.snap(), 1);
    }
}
