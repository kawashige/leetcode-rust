use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

struct MovieRentingSystem {
    movies: HashMap<(i32, i32), i32>,
    rented: HashSet<(i32, i32)>,
    search: HashMap<i32, BinaryHeap<(Reverse<i32>, Reverse<i32>)>>,
    report: BinaryHeap<(Reverse<i32>, Reverse<i32>, Reverse<i32>)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovieRentingSystem {
    fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut movies = HashMap::new();
        let mut search = HashMap::new();
        for e in entries {
            movies.insert((e[0], e[1]), e[2]);
            (*search.entry(e[1]).or_insert(BinaryHeap::new())).push((Reverse(e[2]), Reverse(e[0])));
        }
        Self {
            movies,
            rented: HashSet::new(),
            search,
            report: BinaryHeap::new(),
        }
    }

    fn search(&mut self, movie: i32) -> Vec<i32> {
        if !self.search.contains_key(&movie) {
            return Default::default();
        }
        let mut tmp = Vec::new();
        while let Some((Reverse(p), Reverse(s))) = self.search.get_mut(&movie).unwrap().pop() {
            if self.rented.contains(&(movie, s)) {
                continue;
            }
            if tmp.last() != Some(&(p, s)) {
                tmp.push((p, s));
                if tmp.len() == 5 {
                    break;
                }
            }
        }

        let mut result = Vec::new();
        for (p, s) in tmp {
            self.search
                .get_mut(&movie)
                .unwrap()
                .push((Reverse(p), Reverse(s)));
            result.push(s);
        }
        result
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        self.rented.insert((movie, shop));
        self.report.push((
            Reverse(self.movies[&(shop, movie)]),
            Reverse(shop),
            Reverse(movie),
        ));
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        self.rented.remove(&(movie, shop));
        self.search
            .get_mut(&movie)
            .unwrap()
            .push((Reverse(self.movies[&(shop, movie)]), Reverse(shop)));
    }

    fn report(&mut self) -> Vec<Vec<i32>> {
        let mut tmp = Vec::new();
        while let Some((Reverse(p), Reverse(s), Reverse(m))) = self.report.pop() {
            if !self.rented.contains(&(m, s)) {
                continue;
            }
            if tmp.last() != Some(&(p, s, m)) {
                tmp.push((p, s, m));
                if tmp.len() == 5 {
                    break;
                }
            }
        }

        let mut result = Vec::new();
        for (p, s, m) in tmp {
            self.report.push((Reverse(p), Reverse(s), Reverse(m)));
            result.push(vec![s, m]);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1912() {
        let mut obj = MovieRentingSystem::new(
            3,
            vec![
                vec![0, 1, 5],
                vec![0, 2, 6],
                vec![0, 3, 7],
                vec![1, 1, 4],
                vec![1, 2, 7],
                vec![2, 1, 5],
            ],
        );
        assert_eq!(obj.search(1), vec![1, 0, 2]);
        obj.rent(0, 1);
        obj.rent(1, 2);
        assert_eq!(obj.report(), vec![[0, 1], [1, 2]]);
        obj.drop(1, 2);
        assert_eq!(obj.search(2), vec![0, 1]);
    }
}
