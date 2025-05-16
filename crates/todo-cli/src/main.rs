use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "done")]
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    status: Status,
}

const FILE_PATH: &str = "tasks.json";

#[derive(Parser)]
#[command(name = "Todo CLI")]
#[command(about = "A simple CLI todo app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    List,
    Done { id: usize },
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = load_tasks();

    match cli.command {
        Commands::Add { description } => {
            let task = Task {
                id: tasks.len(),
                description,
                status: Status::Pending,
            };
            tasks.push(task);
            save_tasks(&tasks);
            println!("Task added");
        }
        Commands::List => {
            for task in tasks {
                println!(
                    "{}. [{}] {}",
                    task.id,
                    match task.status {
                        Status::Pending => "_",
                        Status::Done => "✓",
                    },
                    task.description
                )
            }
        }
        Commands::Done { id } => {
            if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
                task.status = Status::Done;
                save_tasks(&tasks);
                println!("Task {} marked as done.", id);
            } else {
                println!("Task {} not found.", id);
            }
        }
    }
}

fn load_tasks() -> Vec<Task> {
    let file = File::open(FILE_PATH).unwrap_or_else(|_| File::create(FILE_PATH).unwrap());
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or(vec![])
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILE_PATH)
        .unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
