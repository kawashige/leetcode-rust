struct Bitset {
    base: bool,
    state: Vec<bool>,
    count: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bitset {
    fn new(size: i32) -> Self {
        Self {
            base: false,
            state: vec![false; size as usize],
            count: 0,
        }
    }

    fn fix(&mut self, idx: i32) {
        if self.base {
            if self.state[idx as usize] {
                self.count += 1;
            }
            self.state[idx as usize] = false;
        } else {
            if !self.state[idx as usize] {
                self.count += 1;
            }
            self.state[idx as usize] = true;
        }
        println!("{:?}, {}, {}", self.state, self.base, self.count);
    }

    fn unfix(&mut self, idx: i32) {
        if self.base {
            if !self.state[idx as usize] {
                self.count -= 1;
            }
            self.state[idx as usize] = true;
        } else {
            if self.state[idx as usize] {
                self.count -= 1;
            }
            self.state[idx as usize] = false;
        }
        println!("{:?}, {}, {}", self.state, self.base, self.count);
    }

    fn flip(&mut self) {
        self.base = !self.base;
        self.count = self.state.len() - self.count;
        println!("{:?}, {}, {}", self.state, self.base, self.count);
    }

    fn all(&self) -> bool {
        self.count == self.state.len()
    }

    fn one(&self) -> bool {
        println!("{:?}, {}, {}", self.state, self.base, self.count);
        0 < self.count
    }

    fn count(&self) -> i32 {
        println!("{:?}, {}, {}", self.state, self.base, self.count);
        self.count as i32
    }

    fn to_string(&self) -> String {
        self.state
            .iter()
            .map(|b| match (self.base, b) {
                (true, true) => '0',
                (true, false) => '1',
                (false, true) => '1',
                (false, false) => '0',
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2166() {
        let mut bs = Bitset::new(1);
        assert!(!bs.all());
        bs.fix(0);
        bs.flip();
        bs.fix(0);
        bs.flip();
        bs.unfix(0);
        assert!(!bs.one());
        assert!(!bs.all());
        assert_eq!(bs.to_string(), "0".to_string());
        bs.flip();
        bs.fix(0);
        assert!(bs.all());

        let mut bs = Bitset::new(5);
        bs.fix(3);
        bs.fix(1);
        bs.flip();
        assert!(!bs.all());
        bs.unfix(0);
        bs.flip();
        assert!(bs.one());
        bs.unfix(0);
        assert_eq!(bs.count(), 2);
        assert_eq!(bs.to_string(), "01010".to_string());
    }
}
