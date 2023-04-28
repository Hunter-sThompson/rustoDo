mod db;
extern crate chrono;
use db::{load_tasks, save_tasks};
use std::io;
use std::fmt;
use chrono::NaiveDate;
use chrono::{DateTime, Utc};


#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Priority {
    Low,
    Medium,
    High,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    title: String,
    description: String,
    due_date: DateTime<Utc>,
    priority: Priority,
    status: bool,
}

impl Task {
    fn new(title: &str, description: &str, due_date: DateTime<Utc>, priority: Priority) -> Self {
        Task {
            title: title.to_string(),
            description: description.to_string(),
            due_date: due_date,
            priority,
            status: false,
        }
    }
}

fn add_task(tasks: &mut Vec<Task>, task: Task) {
    tasks.push(task);
}

fn remove_task(tasks: &mut Vec<Task>, title: &str) {
    tasks.retain(|task| task.title != title);
}

fn show_tasks(tasks: &[Task]) {
    println!("Title          || Due date          || Priority || Status");
    println!("-------------------------------------------------------");
    for task in tasks {
        let status = if task.status { "True" } else { "False" };
        println!("{: <14} || {: <17} || {: <8} || {}", task.title, task.due_date, task.priority, status);
    }
}

fn show_task_details(tasks: &[Task], title: &str) {
    for task in tasks {
        if task.title == title {
            println!("Title: {}", task.title);
            println!("Description: {}", task.description);
            println!("Due date: {}", task.due_date);
            println!("Priority: {}", task.priority);
            println!("Status: {}", if task.status { "Completed" } else { "Incomplete" });
            return;
        }
    }
    println!("Task not found");
}

fn sort_tasks_by_due_date(tasks: &mut Vec<Task>) {
    tasks.sort_by(|a, b| a.due_date.cmp(&b.due_date));
}

fn sort_tasks_by_priority(tasks: &mut Vec<Task>) {
    tasks.sort_by(|a, b| a.priority.cmp(&b.priority));
}

fn main() {
    println!("Welcome to the Todo List!");

    let mut tasks = match load_tasks() {
        Ok(tasks) => tasks,
        Err(err) => {
            eprintln!("Error loading tasks from the database: {}", err);
            vec![]
        }
    };

    let mut tasks = vec![];

    loop {
        println!("Please choose an action:");
        println!("1. Add a task");
        println!("2. Remove a task");
        println!("3. Complete a task");
        println!("4. Show tasks");
        println!("5. Show task details");
        println!("6. Sort tasks by due date");
        println!("7. Sort tasks by priority");
        println!("8. Quit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<u32>() {
            Ok(choice) => {
                match choice {
                    1 => {
                        // Add a task
                        println!("Enter task title:");
                        let mut title = String::new();
                        io::stdin().read_line(&mut title).expect("Failed to read title");

                        println!("Enter task description:");
                        let mut description = String::new();
                        io::stdin().read_line(&mut description).expect("Failed to read description");

                        // Example due date input: "2023-04-20T10:30:00Z"
                        println!("Enter task due date dd/mm/yyyy:");
                        let due_date: DateTime<Utc>;
                        loop {
                            let mut due_date_input = String::new();
                            io::stdin().read_line(&mut due_date_input).expect("Failed to read due date");

                            match NaiveDate::parse_from_str(due_date_input.trim(), "%d/%m/%Y") {
                                Ok(parsed_naive_date) => {
                                    due_date = DateTime::<Utc>::from_utc(parsed_naive_date.and_hms(0, 0, 0), Utc);
                                    break;
                                }
                                Err(_) => {
                                    println!("Invalid date format, please try again");
                                }
                            }

                        }


                        println!("Enter task priority (Low, Medium, High):");
                        let mut priority_input = String::new();
                        io::stdin().read_line(&mut priority_input).expect("Failed to read priority");

                        let priority = match priority_input.trim() {
                            "Low" => Priority::Low,
                            "Medium" => Priority::Medium,
                            "High" => Priority::High,
                            _ => {
                                println!("Invalid priority, setting default to Low");
                                Priority::Low
                            }
                        };

                        let task = Task::new(&title.trim(), &description.trim(), due_date, priority);
                        add_task(&mut tasks, task);
                    }
                    2 => {
                        // Remove a task
                        println!("Enter task title to remove:");
                        let mut title = String::new();
                        io::stdin().read_line(&mut title).expect("Failed to read title");

                        remove_task(&mut tasks, &title.trim());
                    }
                    3 => {
                        // Complete a task
                        println!("Enter task title to mark as complete:");
                        let mut title = String::new();
                        io::stdin().read_line(&mut title).expect("Failed to read title");

                        for task in &mut tasks {
                            if task.title == title.trim() {
                                task.status = true;
                                break;
                            }
                        }
                    }
                    4 => {
                        // Show tasks
                        show_tasks(&tasks);
                    }
                    5 => {
                        println!("Enter task title to show details:");
                        let mut title = String::new();
                        io::stdin().read_line(&mut title).expect("Failed to read title");

                        show_task_details(&tasks, &title.trim());
                    }
                    6 => {
                        // sort task by due date
                        sort_tasks_by_due_date(&mut tasks);

                    }
                    7 => {
                        // sort task by priority
                        sort_tasks_by_priority(&mut tasks);
                    }
                    8 => {
                        // Quit
                        // Save tasks to the database
                        match save_tasks(&tasks) {
                            Ok(_) => {
                                println!("Tasks saved successfully");
                            }
                            Err(err) => {
                                eprintln!("Error saving tasks to the database: {}", err);
                            }
                        }
                        break;
                    }
                    _ => {
                        println!("Invalid choice, please try again");
                    }
                }
            }
            Err(_) => {
                println!("Invalid input, please try again");
            }
        }
    }
}
