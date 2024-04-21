struct Allocator {
    memory: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Allocator {
    fn new(n: i32) -> Self {
        Self {
            memory: vec![0; n as usize],
        }
    }

    fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let mut count = 0;
        for i in 0..self.memory.len() {
            if self.memory[i] != 0 {
                count = 0;
            } else {
                count += 1;
                if count == size as usize {
                    for j in i + 1 - count..=i {
                        self.memory[j] = m_id;
                    }
                    return (i + 1 - count) as i32;
                }
            }
        }
        -1
    }

    fn free(&mut self, m_id: i32) -> i32 {
        let mut count = 0;
        for i in 0..self.memory.len() {
            if self.memory[i] == m_id {
                count += 1;
                self.memory[i] = 0;
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2502() {
        let mut obj = Allocator::new(10);
        assert_eq!(obj.allocate(1, 1), 0);
        assert_eq!(obj.allocate(1, 2), 1);
        assert_eq!(obj.allocate(1, 3), 2);
        assert_eq!(obj.free(2), 1);
        assert_eq!(obj.allocate(3, 4), 3);
        assert_eq!(obj.allocate(1, 1), 1);
        assert_eq!(obj.allocate(1, 1), 6);
        assert_eq!(obj.free(1), 3);
        assert_eq!(obj.allocate(10, 2), -1);
        assert_eq!(obj.free(7), 0);
    }
}
