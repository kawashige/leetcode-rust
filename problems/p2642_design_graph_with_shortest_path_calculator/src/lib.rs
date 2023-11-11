use std::{cmp::Reverse, collections::BinaryHeap};

struct Graph {
    matrix: Vec<Vec<(usize, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut matrix = vec![vec![]; n as usize];
        for edge in edges {
            matrix[edge[0] as usize].push((edge[1] as usize, edge[2]));
        }
        Self { matrix }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        self.matrix[edge[0] as usize].push((edge[1] as usize, edge[2]));
    }

    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), node1 as usize));
        let mut seen = vec![false; self.matrix.len()];

        while let Some((Reverse(cost), node)) = heap.pop() {
            if node == node2 as usize {
                return cost;
            }
            if seen[node] {
                continue;
            }
            seen[node] = true;
            for (next, next_cost) in &self.matrix[node] {
                heap.push((Reverse(cost + next_cost), *next));
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2642() {
        let mut obj = Graph::new(
            4,
            vec![vec![0, 2, 5], vec![0, 1, 2], vec![1, 2, 1], vec![3, 0, 3]],
        );
        assert_eq!(obj.shortest_path(3, 2), 6);
        assert_eq!(obj.shortest_path(0, 3), -1);
        obj.add_edge(vec![1, 3, 4]);
        assert_eq!(obj.shortest_path(0, 3), 6);
    }
}
