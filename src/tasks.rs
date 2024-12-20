use chrono::Local;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use std::io::ErrorKind;

use std::fmt;

use std::fs::{File, OpenOptions};
use std::io::{Error, Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub task: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(task: &str) -> Task {
        Task {
            task: task.to_string(),
            created_at: Utc::now(),
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.task, created_at)
    }
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // open the file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks: Vec<Task> = collect_task(&file)?;

    tasks.push(task);

    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

/// fn to complete tasks
pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut tasks = collect_task(&file)?;

    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task Id"));
    }

    tasks.remove(task_position - 1);

    file.set_len(0)?;

    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

/// fn to print tasks
pub fn list_tasks(journal_path : PathBuf) -> Result<()>{

    // open file
    let file = OpenOptions::new().read(true).open(journal_path)?;

    let tasks = collect_task(&file)?;

    if tasks.is_empty(){
        println!("Tasks is empty!");
    }
    else{

        let mut order = 1;

        for task in tasks{
            println!("{} : {}", order, task.task);

            order +=1;
        }
    }

    Ok(())

}


fn collect_task(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;

    let tasks: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    file.seek(SeekFrom::Start(0))?;

    Ok(tasks)
}
