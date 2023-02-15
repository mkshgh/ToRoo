use std::{io::{self, Write, Read,prelude::*},fs,path::Path,fs::File,fs::OpenOptions, string};
use chrono::{self, DateTime, Utc};
const DATE_FORMAT_STR: &'static str = "%Y-%m-%dT%H:%M:%S%.3fz";
const db_name:&str = "D:\\Onedrive\\Code\\erlang\\ToRoo\\src\\DB\\lists.db";
const temp_db_name:&str = "D:\\Onedrive\\Code\\erlang\\ToRoo\\src\\DB\\lists.db.tmp";
const tsv_separtor:&str = "~|~";
struct taskList {
    uuid: String,
    taskname: String,
    task_priority: char,
}
//time.format(DATE_FORMAT_STR).to_string()
fn main() {
    // let my_tsv = new_task_list();
    // print_task_list(task_name,task_description);
    // write_tsv(my_tsv);
    update_tsv("2023-02-15T09:26:59.870z".to_string(),"2023-02-15T09:26:59.870z~|~NewComplete SOP~|~Complete SOP by this month~|~L".to_string());
    read_tsv(0)

}

fn new_task_list() -> String{
    let time_in_sec = chrono::offset::Utc::now();
    let mut task_name:String = String::from("");
    let mut task_description:String = String::from("");
    print!("Insert a To-Do List: ");
    io::stdout().flush().expect("Issue with Printing Text");
    std::io::stdin().read_line(&mut task_name).unwrap();


    print!("Add Desription(Leave Empty if None): ");
    io::stdout().flush().expect("Issue with Printing Text");
    std::io::stdin().read_line(&mut task_description).unwrap();
    time_in_sec.format(DATE_FORMAT_STR).to_string()+tsv_separtor+&task_name.replace("\r\n", "")+tsv_separtor+&task_description.replace("\r\n","")+tsv_separtor+"L\n"
    // time_in_sec.to_string()+task_name.replace("\r\n","").to_string()+task_description.replace("\r\n","").to_string()
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

fn read_tsv(no_of_elements:u128){
    let file_path:&Path = Path::new(db_name);
    let mut db_content:String = "".to_string();
    let mut db_file:File = File::open(file_path).expect("Could not open file");
    db_file.read_to_string(&mut db_content).unwrap();
    db_file;
    for i in db_content.lines(){
        println!("Start ---------");
        split_tsv(i);
    }
}

fn write_tsv(data:String){
    let file_path:&Path = Path::new(db_name);
    let mut db_file:File = OpenOptions::new().write(true).append(true).open(file_path).expect("Cannot Access DB file.");
    db_file.write_all(data.as_bytes());
}

fn update_tsv(id:String,new_task_list:String) -> bool {
    let file_path:&Path = Path::new(db_name);
    let temp_file_path:&Path = Path::new(temp_db_name);
    File::create(temp_file_path);
    let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;
    let mut temp_file:File = OpenOptions::new().write(true).append(true).open(temp_file_path).expect("Cannot Access DB file.");

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    for data in buffer.lines(){
        if data.starts_with(&id){
            let new_task_list = new_task_list.clone()+"\n";
            temp_file.write_all(new_task_list.as_bytes());
        }
        else {
            let data = data.to_owned()+"\n";
            temp_file.write_all(data.as_bytes());
        }
    }
    drop(file);
    fs::remove_file(file_path).expect("Could not delete File");
    fs::rename(temp_file_path,file_path).expect("Could not Rebuild the DB");
    true
}

fn split_tsv(word:&str)->Vec<String>{
    let mut myVec:Vec<String> = Vec::new();
    let mut data = word.split(tsv_separtor);
    for i in data{
        myVec.push(i.to_owned());
        // Just for test
        println!("{}",i.to_owned())
    }
    myVec
}