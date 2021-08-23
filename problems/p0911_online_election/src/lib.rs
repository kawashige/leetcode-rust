struct TopVotedCandidate {
    top: Vec<i32>,
    times: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut max = 0;
        let mut top_person = 0;

        let mut votes = vec![0; persons.len()];
        let mut top = Vec::with_capacity(persons.len());

        for (p, t) in persons.into_iter().zip(times.iter()) {
            votes[p as usize] += 1;
            if max <= votes[p as usize] {
                max = votes[p as usize];
                top_person = p;
            }

            top.push(top_person);
        }

        Self { top, times }
    }

    fn q(&self, t: i32) -> i32 {
        match self.times.binary_search(&t) {
            Ok(i) => self.top[i],
            Err(i) => self.top[i - 1],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0911() {
        let mut obj =
            TopVotedCandidate::new(vec![0, 1, 1, 0, 0, 1, 0], vec![0, 5, 10, 15, 20, 25, 30]);
        assert_eq!(obj.q(3), 0);
        assert_eq!(obj.q(12), 1);
        assert_eq!(obj.q(25), 1);
        assert_eq!(obj.q(15), 0);
        assert_eq!(obj.q(24), 0);
        assert_eq!(obj.q(8), 1);
    }
}
