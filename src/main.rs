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
    pub fn execute(task: Task, command: Command) {
        match command {
            Add => add(task),
            Remove => remove(task),
            Update => update(task),
            Done => done(task),
            Delete => delete(task),
            _ => println!("not implamented yet"),
        }
    }
    fn add(task: Task) {}
    fn remove(task: Task) {}
    fn update(task: Task) {}

    fn done(task: Task) {}
    fn delete(task: Task) {}
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
