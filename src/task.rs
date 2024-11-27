use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fmt;


#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,

    pub priority: u32,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]",self.text, created_at)
    }
}

impl Task {
    pub fn new(text: String, priority: u32) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at, priority }
    }
}

use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Result, Seek, SeekFrom, Error, ErrorKind};
use std::path::PathBuf;

pub fn add_task( journal_path: PathBuf, task: Task) -> Result<()> {
    // Open file
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    // Consume the file´s content as a vector of tasks.
    let mut tasks = collect_tasks(&file)?;

    file.seek(SeekFrom::Start(0))?;



    
    println!("Se ha agregado a la lista:\nTAREA: {}", task);
    tasks.push(task);

    serde_json::to_writer(file, &tasks)?;
    
    

    Ok(())

}

pub fn complete_task( journal_path: PathBuf, task_position: usize) -> Result<()>{
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;


    // Remove task
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }

    let task = tasks.remove(task_position - 1 );

    // Rewind and truncate the file
    file.seek(SeekFrom::Start(0))?;
    file.set_len(0)?;

    serde_json::to_writer(file, &tasks)?;
    println!("Se ha completado: {}.", task);
    Ok(())
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>>{
    file.seek(SeekFrom::Start(0))?;
    
    let tasks: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    file.seek(SeekFrom::Start(0))?;
    Ok(tasks)
}

pub fn list_tasks( journal_path: PathBuf) -> Result<()> {
    let file: File =  OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let tasks = collect_tasks(&file)?;
    
    if tasks.is_empty() {
        println!("La lista de tareas esta vacia !!")
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            
            println!("{}: {}, priority: {}", order, task,  task.priority);
            order += 1;
        }
    }
    Ok(())
}

pub fn clear_tasks(journal_path: PathBuf) -> Result<()>{
    println!("¿Estás seguro de que quieres borrar todas las tareas? (y/n):");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    println!("Entrada recibida: '{}'", input);

    if input == "y" || input == "yes"{
        println!("Confirmación recibida. Procediendo a borrar las tareas...");
        let file = OpenOptions::new() 
            .write(true)
            .truncate(true)
            .open(journal_path)?;
        file.set_len(0)?;

        println!("La lista de tareas ha sido borrada.");
    } else {
        println!("Operación cancelada.");
    }
    
    
    Ok(())

}
