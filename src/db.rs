use crate::{Task, Priority};
use rusqlite::{Connection, Result, NO_PARAMS};
use chrono::{DateTime, Utc};

pub fn save_tasks(tasks: &[Task]) -> Result<()> {
    let conn = Connection::open("tasks.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            due_date TEXT NOT NULL,
            priority INTEGER NOT NULL,
            status INTEGER NOT NULL
        )",
        NO_PARAMS,
    )?;

    for task in tasks {
        let priority = task.priority as i32;
        conn.execute(
            "INSERT INTO tasks (title, description, due_date, priority, status) VALUES (?1, ?2, ?3, ?4, ?5)",
            &[
                &task.title,
                &task.description,
                &task.due_date.to_rfc3339(),
                &priority,
                &task.status as i32,
            ],
        )?;
    }

    Ok(())
}

pub fn load_tasks() -> Result<Vec<Task>> {
    let conn = Connection::open("tasks.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            due_date TEXT NOT NULL,
            priority INTEGER NOT NULL,
            status INTEGER NOT NULL
        )",
        NO_PARAMS,
    )?;

    let mut stmt = conn.prepare("SELECT title, description, due_date, priority, status FROM tasks")?;
    let task_rows = stmt.query_map(NO_PARAMS, |row| {
        let due_date = row.get::<_, String>(2)?.parse::<DateTime<Utc>>().unwrap();
        let priority = match row.get::<_, i32>(3)? {
            0 => Priority::Low,
            1 => Priority::Medium,
            2 => Priority::High,
            _ => Priority::Low,
        };

        Ok(Task {
            title: row.get(0)?,
            description: row.get(1)?,
            due_date,
            priority,
            status: row.get(4)?,
        })
    })?;

    let tasks = task_rows.collect::<Result<Vec<_>>>()?;

    Ok(tasks)
}

