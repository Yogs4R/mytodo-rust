use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub description: String,
    pub done: bool,
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Gagal membuat JSON");
    fs::write("db.json", json).expect("Gagal menulis file db.json");
}

pub fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("db.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

pub fn add_task(tasks: &mut Vec<Task>, description: String) {
    let task = Task {
        description,
        done: false,
    };
    tasks.push(task);
    save_tasks(tasks); 
    println!("Tugas berhasil ditambahkan.");
}

pub fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("Tidak ada tugas yang tersimpan.");
        return;
    }
    println!("--- Daftar Tugas ---");
    for (i, task) in tasks.iter().enumerate() {
        let status = if task.done { "[x]" } else { "[ ]" };
        println!("{} {} - {}", i + 1, status, task.description);
    }
    println!("--------------------");
}

pub fn mark_done(tasks: &mut Vec<Task>, id: usize) {
    if id == 0 || id > tasks.len() {
        println!("Nomor tugas {} tidak valid.", id);
        return;
    }
    tasks[id - 1].done = true;
    save_tasks(tasks); 
    println!("Tugas {} selesai.", id);
}

pub fn remove_task(tasks: &mut Vec<Task>, id: usize) {
    if id == 0 || id > tasks.len() {
        println!("Nomor tugas {} tidak valid.", id);
        return;
    }
    tasks.remove(id - 1);
    save_tasks(tasks); 
    println!("Tugas {} dihapus.", id);
}