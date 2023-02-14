use std::{io::{self, Write, Read},path::Path,fs::File};
use chrono::{self, DateTime, Utc};
const _DATE_FORMAT_STR: &'static str = "%Y-%m-%dT%H:%M:%S%.3fz";
const db_name:&str = "D:\\Onedrive\\Code\\erlang\\ToRoo\\src\\DB\\lists.db";

struct tasList {
    uuid: String,
    taskname: String,
    task_priority: char,
}
//time.format(DATE_FORMAT_STR).to_string()
fn main() {
    // let (_time, task_name, task_description) = add_task_list();
    // print_task_list(task_name,task_description);
    read_dsv(0)

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

fn read_dsv(no_of_elements:u128){
    let file_path:&Path = Path::new(db_name);
    let mut db_content:String = "".to_string();
    let mut db_file:File = File::open(file_path).expect("Could not open file");
    db_file.read_to_string(&mut db_content).unwrap();
    db_file;
    for i in db_content.lines(){
        println!("Start ---------");
        split_dsv(i);
    }
}

fn split_dsv(word:&str)->Vec<String>{
    let mut myVec:Vec<String> = Vec::new();
    let mut data = word.split("\\,");
    for i in data{
        myVec.push(i.to_owned());
        // Just for test
        println!("{}",i.to_owned())
    }
    myVec
}