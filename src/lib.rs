use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Write, Read, Error, ErrorKind};

pub struct TaskRustler {
  pub tasks: Vec<String>,
  pub task_path: String,
}

impl TaskRustler {
  pub fn new() -> Result<Self, Error> {
    let task_path = String::from("/Users/zajdel/code/task-rustler-cli/tasks.txt");

    let task_file = OpenOptions::new()
      .write(true)
      .read(true)
      .create(true)
      .open(&task_path)?;

    let mut buf_reader = BufReader::new(&task_file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let tasks = contents.lines().map(str::to_string).collect();

    Ok(Self {
      tasks,
      task_path,
    })
  }

  pub fn list(&self) -> Result<String, Error> {
    Ok(self.tasks.join("\n"))
  }

  pub fn add(&self, args: &[String]) -> Result<(), Error> {
    if args.is_empty() {
      return Err(Error::new(ErrorKind::InvalidInput, "No items provided to add"));
    }

    let task_file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(&self.task_path)?;

    let mut buffer = BufWriter::new(task_file);
    for arg in args {
      if arg.trim().is_empty() {
        continue;
      }

      let line = format!("[ ] {}\n", arg);
      buffer.write_all(line.as_bytes())?;
    }

    buffer.flush()?;
    Ok(())
  }

  pub fn sort(&mut self) -> Result<(), Error> {
    self.tasks.sort();

    let sorted = self.tasks.join("\n");

    let mut file = OpenOptions::new()
      .write(true)
      .truncate(true)
      .open(&self.task_path)?;

    file.write_all(sorted.as_bytes())?;

    Ok(())
  }

  pub fn clear(&self) -> Result<(), Error> {
    OpenOptions::new()
      .write(true)
      .truncate(true)
      .open(&self.task_path)?;

    Ok(())
  }
}