use std::{io::{self, Write}};
use chrono::{self, DateTime, Utc};
const _DATE_FORMAT_STR: &'static str = "%Y-%m-%dT%H:%M:%S%.3fz";
//time.format(DATE_FORMAT_STR).to_string()
fn main() {
    let mstr = "List Name: has following tasks:";
    println!("{}",mstr.len());
    let (_time, list_name) = ask_for_list();
    let task_vector = ask_tasks();
    printTaskList(list_name,task_vector);
}

fn ask_for_list() -> (DateTime<Utc>,String){
    let time_in_sec = chrono::offset::Utc::now();
    print!("Insert a To-Do List: ");
    io::stdout().flush();
    let mut todo_list:String = String::from("");
    std::io::stdin().read_line(&mut todo_list).unwrap();
    (time_in_sec,todo_list)
}
fn ask_for_list_test() -> (DateTime<Utc>,String){
    let time_in_sec = chrono::offset::Utc::now();
    let mut todo_list:String = String::from("List 1");
    (time_in_sec,todo_list)
}
fn ask_tasks()-> Vec<String>{
    println!("Add Tasks to the lists (Leave Blank and Press Enter to Finish):");
    let mut task_vector:Vec<String> = Vec::new();
    loop{
        print!("- ");
        let mut myWorks = String::from("");
        io::stdout().flush();
        std::io::stdin().read_line(&mut myWorks).unwrap();
        if myWorks=="\r\n"{
            break;
        }
        else{
            task_vector.push(myWorks.trim().to_owned());
            let myWorks="";
        }
    }
    task_vector
}

fn printTaskList(list_name:String,task_vector:Vec<String>){
    println!("\nList Name:{} has following {} tasks:",list_name,task_vector.len().to_string());
    let decorators = "-".repeat(35+task_vector.len());
    println!("{}",decorators);
    for tasks in &task_vector{
        println!("-> {}",tasks);
    }
}