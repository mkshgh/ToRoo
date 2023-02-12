use std::{io::{self, Write}};
use chrono::{self, DateTime, Utc};
const _DATE_FORMAT_STR: &'static str = "%Y-%m-%dT%H:%M:%S%.3fz";
//time.format(DATE_FORMAT_STR).to_string()
fn main() {
    let mstr = "List Name: has following tasks:";
    println!("{}",mstr.len());
    let (_time, task_name, task_description) = add_task_list();
    print_task_list(task_name,task_description);
}

fn add_task_list() -> (DateTime<Utc>,String,String){
    let time_in_sec = chrono::offset::Utc::now();
    let mut task_name:String = String::from("");
    let mut task_description:String = String::from("");

    print!("Insert a To-Do List: ");
    io::stdout().flush().expect("Issue with Printing Text");
    std::io::stdin().read_line(&mut task_name).unwrap();


    print!("Add Desription(Leave Empty if None): ");
    io::stdout().flush().expect("Issue with Printing Text");
    std::io::stdin().read_line(&mut task_description).unwrap();

    (time_in_sec,task_name.replace("\r\n",""),task_description.replace("\r\n",""))
}



fn print_task_list(task_name:String,task_description:String){
    println!("\nTask Name: {}",task_name);
    let decorators = "-".repeat(11+task_name.len());
    println!("{}",decorators);
    // Just for making it look good
    match task_description.len(){
        0 => println!("{task_name}"),
         _ => println!("{task_description}"),
    }
}