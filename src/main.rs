mod cmdline;

use crate::cmdline::Commands;
use clap::Parser;
use std::io::Write;

fn main() -> anyhow::Result<()> {
    let args = cmdline::Args::parse();
    let mut dir = match std::env::home_dir() {
        Some(d) => d,
        None => panic!("No home dir detected!"),
    };

    dir.push(".cache/todo-rs/list.file");
    if !dir.exists() {
        std::fs::create_dir_all(&dir.parent().unwrap())?;
        std::fs::File::create(&dir)?;
    }

    let mut active_todos = vec![];
    let mut completed_todos = vec![];
    {
    let todolist = std::fs::read_to_string(&dir)?;
    if !todolist.is_empty() {
        let mut active = true;
        for line in todolist.lines() {
            if line.is_empty() {
                active = false;
                continue;
            }

            if active {
                active_todos.push(line.to_string());
            } else {
                completed_todos.push(line.to_string());
            }
        }
    }
    }
    
    let command = args.command.unwrap_or(Commands::List { all: false });
    match command {
        Commands::Insert { item, descritpion } => {
            println!("Insert {:?}", item);
            active_todos.push(item);
            if descritpion.is_some() {
                println!("Desc: {:?}", descritpion);
            }
        }
        Commands::Delete { item } => {
            if let Some(todo) = active_todos.iter().position(|x| *x == item) {
                active_todos.remove(todo);
            } else if let Some(todo) = completed_todos.iter().position(|x| *x == item) {
                completed_todos.remove(todo);
            }

            println!("Delete {:?}", item);
        }
        Commands::Complete { item } => {
            if let Some(todo) = active_todos.iter().position(|x| *x == item) {
                active_todos.remove(todo);
                completed_todos.push(item.to_string());
                println!("Complete {:?}", item);
            } else {
                println!("{} was not in the active todo list.", item)
            }
        }
        Commands::List { all } => {
            println!("Active\n---------------");
            for todo in active_todos.iter() {
                println!("{}", todo);
            }
            if all {
                println!("\nCompleted\n---------------");
                for todo in completed_todos.iter() {
                    println!("{}", todo);
                }
            }
        }
        Commands::Clear => {
            active_todos.clear();
            completed_todos.clear();
            println!("Todo list cleared");
        }
    }

    if active_todos.is_empty() && completed_todos.is_empty() {
        std::fs::OpenOptions::new().write(true).truncate(true).open(dir)?;
        return Ok(())
    } 

    let mut content = active_todos.join("\n");
    content.push_str("\n");
    let complete_content = completed_todos.join("\n");
    let mut file = std::fs::OpenOptions::new()
                         .write(true)
                         .truncate(true)
                         .open(dir)?;
    file.write(content.as_bytes())?;
    file.write(complete_content.as_bytes())?;

    Ok(())
}
