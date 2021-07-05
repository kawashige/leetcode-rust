struct ExamRoom {
    n: i32,
    seated: Vec<i32>,
}

impl ExamRoom {
    fn new(n: i32) -> Self {
        Self {
            n,
            seated: Vec::new(),
        }
    }

    fn seat(&mut self) -> i32 {
        if self.seated.is_empty() {
            self.seated.push(0);
            0
        } else {
            let mut max_d = -1;
            let mut max_i = 0;
            let mut insert_i = 0;

            if self.seated[0] != 0 {
                max_d = self.seated[0] - 1;
                max_i = 0;
            }

            for i in 1..self.seated.len() {
                if self.seated[i] - self.seated[i - 1] == 1 {
                    continue;
                }
                let d = (self.seated[i] - self.seated[i - 1]) / 2 - 1;
                if d > max_d {
                    max_d = d;
                    max_i = self.seated[i - 1] + max_d + 1;
                    insert_i = i;
                }
            }

            if self.seated.last().unwrap() != &(self.n - 1)
                && self.n - 1 - self.seated.last().unwrap() - 1 > max_d
            {
                max_i = self.n - 1;
                insert_i = self.seated.len();
            }

            self.seated.insert(insert_i, max_i);
            max_i
        }
    }

    fn leave(&mut self, p: i32) {
        self.seated.remove(self.seated.binary_search(&p).unwrap());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0855() {
        // let mut obj = ExamRoom::new(10);
        // assert_eq!(obj.seat(), 0);
        // assert_eq!(obj.seat(), 9);
        // assert_eq!(obj.seat(), 4);
        // assert_eq!(obj.seat(), 2);
        // obj.leave(4);
        // assert_eq!(obj.seat(), 5);

        // let mut obj = ExamRoom::new(4);
        // assert_eq!(obj.seat(), 0);
        // assert_eq!(obj.seat(), 3);
        // assert_eq!(obj.seat(), 1);
        // assert_eq!(obj.seat(), 2);
        // obj.leave(1);
        // obj.leave(3);
        // assert_eq!(obj.seat(), 1);

        let mut obj = ExamRoom::new(1000000000);
        assert_eq!(obj.seat(), 0);
        obj.leave(0);
        assert_eq!(obj.seat(), 0);
        obj.leave(0);
        assert_eq!(obj.seat(), 0);
        obj.leave(0);
        assert_eq!(obj.seat(), 0);
        obj.leave(0);
        assert_eq!(obj.seat(), 0);
        obj.leave(0);
    }
}
