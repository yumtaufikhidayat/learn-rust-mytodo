use clap::{Parser, Subcommand};

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
    List,
    /// Menandai tugas dengan nomor tertentu sebagai selesai
    Done {
        id: usize
    },
    /// Menghapus tugas dengan nomor tertentu
    Remove {
        id: usize
    },
}

pub struct Task {
    pub desc: String,
    pub done: bool,
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
pub fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("Belum ada tugas.");
        return;
    }

    println!("Daftar Tugas:");
    for (i, task) in tasks.iter().enumerate() {
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
    let mut tasks: Vec<Task> = Vec::new();
    match cli.command {
        Commands::Add { description } => add_task(&mut tasks, description),
        Commands::List => list_tasks(&tasks),
        Commands::Done { id } => mark_done(&mut tasks, id),
        Commands::Remove { id } => remove_task(&mut tasks, id),
    }
    Ok(())
}