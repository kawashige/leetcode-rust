const M: i64 = 1_000_000_007;
struct Fancy {
    sums: Vec<i32>,
    muls: Vec<i32>,
    vals: Vec<i32>,
}
pub fn modinv(a: i32) -> usize {
    let mut a = a as i64;
    let m = 1_000_000_007;
    let mut b = m;
    let mut u = 1;
    let mut v = 0;
    while b > 0 {
        let t = a / b;
        a -= t * b;
        std::mem::swap(&mut a, &mut b);
        u -= t * v;
        std::mem::swap(&mut u, &mut v);
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    u as usize
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Fancy {
    fn new() -> Self {
        Self {
            sums: vec![0],
            muls: vec![1],
            vals: Vec::new(),
        }
    }

    fn append(&mut self, val: i32) {
        self.sums.push(*self.sums.last().unwrap());
        self.muls.push(*self.muls.last().unwrap());
        self.vals.push(val as i32);
    }

    fn add_all(&mut self, inc: i32) {
        let i = self.sums.len() - 1;
        self.sums[i] = ((self.sums[i] as i64 + inc as i64) % M) as i32;
    }

    fn mult_all(&mut self, m: i32) {
        let i = self.sums.len() - 1;
        self.sums[i] = ((self.sums[i] as i64 * m as i64) % M) as i32;
        self.muls[i] = ((self.muls[i] as i64 * m as i64) % M) as i32;
    }

    fn get_index(&self, idx: i32) -> i32 {
        if (self.vals.len() as i32) <= idx {
            return -1;
        }
        let ao = (*self.muls.last().unwrap() as i64 * modinv(self.muls[idx as usize]) as i64) % M;

        let bo =
            (*self.sums.last().unwrap() as i64 - self.sums[idx as usize] as i64 * ao % M + M) % M;
        let result = (ao * self.vals[idx as usize] as i64 % M + bo) % M;
        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3622() {
        let mut obj = Fancy::new();
        obj.append(2);
        obj.add_all(3);
        obj.append(7);
        obj.mult_all(2);
        assert_eq!(obj.get_index(0), 10);
        obj.add_all(3);
        obj.append(10);
        obj.mult_all(2);
        assert_eq!(obj.get_index(0), 26);
        assert_eq!(obj.get_index(1), 34);
        assert_eq!(obj.get_index(2), 20);
    }
}
