use std::error::Error;

use sqlx::Connection;
use sqlx::Row;

#[derive(Debug, Clone)]
struct Task {
    pub id: String,
    pub time: i32,
    pub data: String,
}

struct TodoList {
    dayly_list: Vec<Task>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { dayly_list: vec![] }
    }

    pub fn execute(&mut self, command: Command) -> Result<(), Box<dyn std::error::Error>> {
        match command {
            Command::Add(task) => {
                self.add(task)?;
                Ok(())
            }
            Command::Remove(task) => {
                self.remove(&task)?;
                Ok(())
            }
            Command::Update(task) => {
                self.update(task);
                Ok(())
            }
            Command::Done(task) => {
                self.done(task);
                Ok(())
            }
            Command::Delete(task) => {
                self.delete(task);
                Ok(())
            }
        }
    }

    async fn create(&self, task: &Task, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let query = "INSERT INTO task (id, time, data) VALUES ($1, $2, $3)";
        sqlx::query(query)
            .bind(&task.id)
            .bind(&task.time)
            .bind(&task.data)
            .execute(pool)
            .await?;
        Ok(())
    }
    async fn get_tasks(&mut self, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let query = "SELECT id, time, data FROM task";
        let rows = sqlx::query(query).fetch_all(pool).await?;
        for row in rows.iter() {
            self.dayly_list.push(Task {
                id: row.get("id"),
                time: row.get("time"),
                data: row.get("data"),
            });
        }

        Ok(())
    }

    async fn open_database_connection(&self) -> Option<bool> {
        Some(true)
    }

    fn add(&mut self, task: Task) -> Result<(), &'static str> {
        self.dayly_list.push(task);
        Ok(())
    }

    fn remove(&mut self, task: &Task) -> Result<(), &'static str> {
        self.dayly_list.retain(|x| x.id != task.id);
        Ok(())
    }

    fn print_list(&self) {
        for i in self.dayly_list.iter() {
            println!("{} {} {}", i.id, i.time, i.data);
        }
    }

    fn update(&mut self, _task: Task) {}
    fn done(&mut self, _task: Task) {}
    fn delete(&mut self, _task: Task) {}
}

enum Command {
    Add(Task),
    Remove(Task),
    Update(Task),
    Done(Task),
    Delete(Task),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "postgres://dbuser:mysecretpassword@localhost:5432/todoapp";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let res = sqlx::query("SELECT 1 + 1 as sum").fetch_one(&pool).await?;

    let sum: i32 = res.get("sum");

    println!("{}", sum);

    let mut td = TodoList::new();

    let t = Task {
        id: "12_32_234_33".to_string(),
        time: 2,
        data: "hello".to_string(),
    };
    let t = Task {
        id: "13".to_string(),
        time: 2,
        data: "hello".to_string(),
    };
    //td.create(&t, &pool).await?;
    td.get_tasks(&pool).await?;
    td.print_list();

    Ok(())
}
