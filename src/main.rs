use std::error::Error;

use chrono::prelude::*;
use sqlx::Connection;
use sqlx::Row;
use std::io;
use std::ops::Add;
use tokio::task;

#[derive(Debug, Clone)]

struct Task {
    pub id: String,
    pub time: i64,
    pub name: String,
    pub data: String,
}

struct TaskInput {
    pub name: String,
    pub data: String,
}

struct TodoList {
    dayly_list: Vec<Task>,
    counter: i64,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            dayly_list: vec![],
            counter: 0,
        }
    }

    pub fn execute(&mut self, command: Command) -> Result<(), Box<dyn std::error::Error>> {
        match command {
            Command::Add(task_input) => {
                self.counter += 1;
                let task = Task {
                    id: self.counter.to_string(),
                    time: Utc::now().timestamp_millis(),
                    name: task_input.name,
                    data: task_input.data,
                };
                self.add(task)?;
                Ok(())
            }
            Command::Remove(task_input) => {
                //self.remove(task)?;
                Ok(())
            }
            Command::Update(task_input) => {
                //self.update(task);
                Ok(())
            }
            Command::Done(task_input) => {
                //self.done(task);
                Ok(())
            }
            Command::Delete(task_input) => {
                //self.delete(task);
                Ok(())
            }
        }
    }

    async fn create(&self, task: &Task, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let query = "INSERT INTO tasks (id, time, name, data) VALUES ($1, $2, $3, $4)";
        sqlx::query(query)
            .bind(&task.id)
            .bind(&task.time)
            .bind(&task.name)
            .bind(&task.data)
            .execute(pool)
            .await?;
        Ok(())
    }
    async fn get_tasks(&mut self, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let query = "SELECT id, time, name, data FROM tasks";
        let rows = sqlx::query(query).fetch_all(pool).await?;
        for row in rows.iter() {
            self.dayly_list.push(Task {
                id: row.get("id"),
                time: row.get("time"),
                name: row.get("name"),
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
            println!("{} {} {} {}", i.id, i.time, i.name, i.data);
        }
    }

    fn update(&mut self, _task: Task) {}
    fn done(&mut self, _task: Task) {}
    fn delete(&mut self, _task: Task) {}
}

enum Command {
    Add(TaskInput),
    Remove(TaskInput),
    Update(TaskInput),
    Done(TaskInput),
    Delete(TaskInput),
}

pub fn parse_to_command(line: String) -> Result<Command, Box<dyn Error>> {
    let tokens: Vec<&str> = line.split_whitespace().collect();

    let (command_token, task_tokens) = tokens.split_first().ok_or("Empty input")?;

    let task_input = parse_to_task(task_tokens)?;

    match command_token.to_lowercase().as_str() {
        "add" => Ok(Command::Add(task_input)),
        "remove" => Ok(Command::Remove(task_input)),
        "update" => Ok(Command::Update(task_input)),
        "done" => Ok(Command::Done(task_input)),
        "delete" => Ok(Command::Delete(task_input)),
        _ => Err("invalid command".into()),
    }
}

fn parse_to_task(task_tokens: &[&str]) -> Result<TaskInput, Box<dyn Error>> {
    let (name, data) = task_tokens
        .split_first()
        .ok_or("missing task description")?;

    Ok(TaskInput {
        name: name.to_string(),
        data: data.join(" "),
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "postgres://dbuser:mysecretpassword@localhost:5432/todoapp";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let mut tl = TodoList::new();

    let mut stop = false;
    let mut commands: Vec<String> = vec![];

    while !stop {
        let mut command: String = String::new();
        std::io::stdin()
            .read_line(&mut command)
            .expect("Unable to read Stdin");

        //commands.push(command.clone());
        let command = parse_to_command(command)?;
        if tl.execute(command).is_ok() {
            break;
        }
    }

    //td.create(&t, &pool).await?;
    tl.get_tasks(&pool).await?;
    tl.print_list();

    Ok(())
}
