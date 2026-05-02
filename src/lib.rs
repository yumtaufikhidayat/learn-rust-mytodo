use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

const TASKS_FILE: &str = "tasks.json";

fn get_tasks_file() -> std::path::PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    path.push(TASKS_FILE);
    path
}

#[derive(Parser)]
#[command(name = "mytodo", version = "1.0", about = "Aplikasi to-do list CLI sederhana")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    /// Menambah tugas baru dengan deskripsi tertentu
    Add {
        description: String
    },
    /// Menampilkan semua tugas
    List {
        #[arg(long)]
        pending: bool
    },
    /// Menandai tugas dengan nomor tertentu sebagai selesai
    Done {
        id: usize
    },
    /// Menghapus tugas dengan nomor tertentu
    Remove {
        id: usize
    },
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub desc: String,
    pub done: bool,
}

pub fn load_tasks() -> Result<Vec<Task>, anyhow::Error> {
    let path = get_tasks_file();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = std::fs::read_to_string(path)?;
    let tasks = serde_json::from_str::<Vec<Task>>(&data)?;
    Ok(tasks)
}

pub fn save_tasks(tasks: &Vec<Task>) -> Result<(), anyhow::Error> {
    let path = get_tasks_file();
    let data = serde_json::to_string(tasks)?;
    std::fs::write(path, data)?;
    Ok(())
}

/// Menambah tugas baru ke daftar
pub fn add_task(tasks: &mut Vec<Task>, description: String) {
    let task = Task {
        desc: description,
        done: false,
    };
    tasks.push(task);
    println!("Tugas baru ditambahkan.");
}

/// Menampilkan semua tugas dalam daftar
pub fn list_tasks(tasks: &Vec<Task>, pending_only: bool) {
    if tasks.is_empty() {
        println!("Belum ada tugas.");
        return;
    }

    let filtered: Vec<_> = tasks.iter().enumerate()
        .filter(|(_, task)| !pending_only || !task.done)
        .collect();

    if filtered.is_empty() {
        println!("Tidak ada tugas yang belum selesai.");
        return;
    }

    println!("Daftar Tugas:");
    for (i, task) in filtered {
        let status = if task.done { "[x]" } else { "[ ]" };
        println!("{}. {} {}", i + 1, status, task.desc);
    }
}

/// Menandai tugas selesai
pub fn mark_done(tasks: &mut Vec<Task>, id: usize) {
    if id == 0 || id > tasks.len() {
        println!("Error: Nomor tugas {} tidak valid!.", id);
        return;
    }

    let index = id - 1;
    tasks[index].done = true;
    println!("Tugas {} telah ditandai selesai.", id);
}

/// Menghapus tugas
pub fn remove_task(tasks: &mut Vec<Task>, id: usize) {
    if id == 0 || id > tasks.len() {
        println!("Error: Nomor tugas {} tidak valid!.", id);
        return;
    }

    let index = id - 1;
    tasks.remove(index);
    println!("Tugas {} telah dihapus.", id);
}

pub fn run(cli: Cli) -> Result<(), anyhow::Error>{
    let mut tasks: Vec<Task> = load_tasks()?;
    match cli.command {
        Commands::Add { description } => {
            add_task(&mut tasks, description);
            save_tasks(&tasks)?;
        },
        Commands::List { pending } => list_tasks(&tasks, pending),
        Commands::Done { id } => {
            mark_done(&mut tasks, id);
            save_tasks(&tasks)?;
        },
        Commands::Remove { id } => {
            remove_task(&mut tasks, id);
            save_tasks(&tasks)?;
        },
    }
    Ok(())
}