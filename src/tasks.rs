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