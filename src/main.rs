use chrono::prelude::*;

struct TodoList {
    time: DateTime<Local>,
    tasks: Vec<Task>,
}
impl TodoList {
    pub fn new() -> TodoList {
        let _time = Local::now();
        return TodoList {
            time: _time,
            tasks: vec![],
        };
    }
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
}

struct Task {
    id: i32,
    name: String,
    start: i32,
    end: i32,
}
impl Task {
    pub fn printer(&self) {
        println!(
            "id:{} name:{} start:{} end:{}",
            self.id, self.name, self.start, self.end
        )
    }
}

fn main() {
    let mut todo = TodoList::new();
    let task: Task = Task {
        id: 1,
        name: String::from("new"),
        start: 12,
        end: 14,
    };
    let rtodo = &mut todo;

    todo.add_task(task);

    println!("Hello, world! {}", todo.time);
    for t in todo.tasks.iter() {
        t.printer();
    }
}
