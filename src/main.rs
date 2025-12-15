use clap::{Parser, Subcommand};
mod tasks;
use tasks::{add_task, list_tasks, mark_done, remove_task, load_tasks, Task};

#[derive(Parser)]
#[command(name = "Mytodo", version = "1.0", about = "Simple  CLI Todo App")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Tambah tugas baru
    Add { description: String },
    /// Daftar semua tugas
    List,
    /// Tandai tugas sebagai selesai
    Done { id: usize },
    /// Hapus tugas
    Remove { id: usize },
}

fn main() {
    let cli = Cli::parse(); // parsing argumen CLI
    
    let mut tasks: Vec<Task> = load_tasks(); // muat tugas

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
