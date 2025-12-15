struct Task {
    id: String,
    time: i32,
    data: String,
}

struct TodoList {
    dayly_list: Vec<Task>,
}

impl TodoList {
    pub fn new() -> Self {
        return TodoList { dayly_list: vec![] };
    }
    pub fn execute(&self, task: Task, command: Command) {
        match command {
            Add => self.add(task),
            Remove => self.remove(task),
            Update => self.update(task),
            Done => self.done(task),
            Delete => self.delete(task),
            _ => println!("not implamented yet"),
        }
    }
    fn open_database_connection(&self) -> Option<bool> {
        return Some(true);
    }
    fn add(&mut self, task: Task) -> Option<bool> {
        match self.open_database_connection() {
            Some(true) => {
                self.dayly_list.push(task);
                Some(true)
            }
            Some(false) | None => {
                println!("failed to open database");
                None
            }
        }
    }
    fn remove(&mut self, task: Task) -> Option<bool> {
        match self.open_database_connection() {
            Some(true) => {
                if let Some(index) = self.dayly_list.iter().position(|x| x == &task) {
                    self.dayly_list.remove(index);
                }
                Some(true)
            }
            Some(false) | None => {
                println!("failed to open database");
                None
            }
        }
    }
    fn update(task: Task) -> Option<bool> {}
    fn done(task: Task) -> Option<bool> {}
    fn delete(task: Task) -> Option<bool> {}
}

enum Command {
    Add(Task),
    Remove(Task),
    Update(Task),
    Done(Task),
    Delete(Task),
}

fn main() {
    println!("Hello world");

    let td: TodoList::new();
    let t: Task = Task {
        id: "12",
        time: 2,
        data: "hello",
    };
    td.execute()
}
