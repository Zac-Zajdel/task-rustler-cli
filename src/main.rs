use std::{env, io::Error};

use task_rustler::TaskRustler;

fn main() -> Result<(), Error> {
    let mut task_rustler = TaskRustler::new().expect("Failed To Create Instance");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];

        match &command[..] {
            "list" => {
                let content = task_rustler.list()?;
                println!("{}", content);
            }
            "add" => {
                task_rustler.add(&args[2..])?
            }
            "sort" => {
                task_rustler.sort()?;
            }
            "clear" => {
                task_rustler.clear()?
            }
            _ => println!("Not A Valid Argument")
        }
    } else {
        println!("Not able to grab arguments");
    }

    Ok(())
}
