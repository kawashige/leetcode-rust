use std::collections::BinaryHeap;

struct TaskManager {
    tasks: BinaryHeap<(i32, i32, i32, i32)>,
    status: Vec<i32>,
    user_ids: Vec<i32>,
    version: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let version = tasks.len() as i32;
        let mut tasks_ = BinaryHeap::new();
        let mut status = vec![-1; 100_001];
        let mut user_ids = vec![-1; 100_001];
        for i in 0..tasks.len() {
            tasks_.push((tasks[i][2], tasks[i][1], tasks[i][0], i as i32));
            user_ids[tasks[i][1] as usize] = tasks[i][0];
            status[tasks[i][1] as usize] = i as i32;
        }
        Self {
            tasks: tasks_,
            status,
            user_ids,
            version,
        }
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.tasks.push((priority, task_id, user_id, self.version));
        self.user_ids[task_id as usize] = user_id;
        self.status[task_id as usize] = self.version;
        self.version += 1;
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        self.status[task_id as usize] = new_priority;
        self.tasks.push((
            new_priority,
            task_id,
            self.user_ids[task_id as usize],
            self.version,
        ));
        self.status[task_id as usize] = self.version;
        self.version += 1;
    }

    fn rmv(&mut self, task_id: i32) {
        self.status[task_id as usize] = -1;
    }

    fn exec_top(&mut self) -> i32 {
        while let Some((_, t, u, v)) = self.tasks.pop() {
            if self.status[t as usize] != v {
                continue;
            }
            return u;
        }
        -1
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3408() {
        let mut obj = TaskManager::new(vec![vec![1, 101, 10], vec![2, 102, 20], vec![3, 103, 15]]);
        obj.add(4, 104, 5);
        obj.edit(102, 8);
        assert_eq!(obj.exec_top(), 3);
        obj.rmv(101);
        obj.add(5, 105, 15);
        assert_eq!(obj.exec_top(), 5);
    }
}
