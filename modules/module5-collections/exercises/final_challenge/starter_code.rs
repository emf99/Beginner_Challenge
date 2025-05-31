use std::io::{self, Write};

mod models {
    #[derive(Debug)]
    pub enum TaskStatus {
        Pending,
        Completed,
    }

    #[derive(Debug)]
    pub struct Task {
        pub id: u32,
        pub title: String,
        pub description: String,
        pub due_date: Option<String>,
        pub status: TaskStatus,
    }
}

mod manager {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Write};
    use crate::models::{Task, TaskStatus};

    pub struct TaskManager {
        pub tasks: Vec<Task>,
        pub next_id: u32,
    }

    impl TaskManager {
        pub fn new() -> Self {
            TaskManager {
                tasks: Vec::new(),
                next_id: 1,
            }
        }

        pub fn add_task(&mut self, title: String, description: String, due_date: Option<String>) -> &Task {
            let task = Task {
                id: self.next_id,
                title,
                description,
                due_date,
                status: TaskStatus::Pending,
            };
            self.next_id += 1;
            self.tasks.push(task);
            self.tasks.last().unwrap()
        }

        pub fn list_tasks(&self) {
            if self.tasks.is_empty() {
                println!("No tasks available.");
            } else {
                for task in &self.tasks {
                    println!(
                        "[#{}] {} - {} (Due: {}) [{}]",
                        task.id,
                        task.title,
                        task.description,
                        task.due_date.as_deref().unwrap_or("No due date"),
                        match task.status {
                            TaskStatus::Pending => "Pending",
                            TaskStatus::Completed => "Completed",
                        }
                    );
                }
            }
        }

        pub fn complete_task(&mut self, id: u32) -> bool {
            for task in &mut self.tasks {
                if task.id == id {
                    task.status = TaskStatus::Completed;
                    return true;
                }
            }
            false
        }

        pub fn delete_task(&mut self, id: u32) -> bool {
            if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
                self.tasks.remove(index);
                return true;
            }
            false
        }

        pub fn save_to_file(&self, filename: &str) -> Result<(), std::io::Error> {
            let mut file = File::create(filename)?;
            for task in &self.tasks {
                let due = task.due_date.clone().unwrap_or_default();
                let status = match task.status {
                    TaskStatus::Pending => "Pending",
                    TaskStatus::Completed => "Completed",
                };
                writeln!(file, "{}|{}|{}|{}|{}", task.id, task.title, task.description, due, status)?;
            }
            Ok(())
        }

        pub fn load_from_file(&mut self, filename: &str) -> Result<(), std::io::Error> {
            let file = File::open(filename)?;
            let reader = BufReader::new(file);
            self.tasks.clear();
            self.next_id = 1;

            for line in reader.lines() {
                let line = line?;
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() != 5 {
                    continue;
                }
                let id = parts[0].parse().unwrap_or(self.next_id);
                let title = parts[1].to_string();
                let description = parts[2].to_string();
                let due_date = if parts[3].is_empty() { None } else { Some(parts[3].to_string()) };
                let status = if parts[4] == "Completed" {
                    TaskStatus::Completed
                } else {
                    TaskStatus::Pending
                };

                self.tasks.push(Task {
                    id,
                    title,
                    description,
                    due_date,
                    status,
                });

                self.next_id = id + 1;
            }

            Ok(())
        }
    }
}

use manager::TaskManager;

fn main() {
    let mut task_manager = TaskManager::new();

    loop {
        println!("\nEnter command: Add;title;desc;due | List | Complete;id | Delete;id | Save;file | Load;file | Quit");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let parts: Vec<&str> = input.split(';').collect();

        match parts[0].to_lowercase().as_str() {
            "add" => {
                if parts.len() >= 3 {
                    let title = parts[1].to_string();
                    let desc = parts[2].to_string();
                    let due = if parts.len() > 3 { Some(parts[3].to_string()) } else { None };
                    let task = task_manager.add_task(title, desc, due);
                    println!("✅ Task added: {}", task.title);
                } else {
                    println!("❌ Add requires: title;description;[due]");
                }
            }
            "list" => task_manager.list_tasks(),
            "complete" => {
                if let Ok(id) = parts.get(1).unwrap_or(&"").parse() {
                    if task_manager.complete_task(id) {
                        println!("✅ Task {} completed.", id);
                    } else {
                        println!("❌ Task not found.");
                    }
                }
            }
            "delete" => {
                if let Ok(id) = parts.get(1).unwrap_or(&"").parse() {
                    if task_manager.delete_task(id) {
                        println!("🗑️ Task {} deleted.", id);
                    } else {
                        println!("❌ Task not found.");
                    }
                }
            }
            "save" => {
                if let Some(fname) = parts.get(1) {
                    if let Err(e) = task_manager.save_to_file(fname) {
                        println!("❌ Failed to save: {}", e);
                    } else {
                        println!("💾 Tasks saved to {}.", fname);
                    }
                }
            }
            "load" => {
                if let Some(fname) = parts.get(1) {
                    if let Err(e) = task_manager.load_from_file(fname) {
                        println!("❌ Failed to load: {}", e);
                    } else {
                        println!("📂 Tasks loaded from {}.", fname);
                    }
                }
            }
            "quit" => {
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❓ Unknown command."),
        }
    }
}
