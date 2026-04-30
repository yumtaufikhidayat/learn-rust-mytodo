mod tasks;

use clap::{Parser, Subcommand};
use tasks::{Task, add_task, list_tasks, mark_done, remove_task};

#[derive(Parser)]
#[command(name = "mytodo", version = "1.0", about = "Aplikasi to-do list CLI sederhana")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
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

fn main() {
    let cli = Cli::parse(); // parsing argumen CLI menjadi struct

    // Inisialisasi daftar tugas (untuk sementara, kosong di awal)
    let mut tasks: Vec<Task> = Vec::new();

    match cli.command {
        Commands::Add { description } => {
            add_task(&mut tasks, description);
        }
        Commands::List => {
            list_tasks(&tasks);
        }
        Commands::Done { id } => {
            mark_done(&mut tasks, id);
        }
        Commands::Remove { id } => {
            remove_task(&mut tasks, id);
        }
    }
}
