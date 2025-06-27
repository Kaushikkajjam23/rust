use std::fs::read_to_string;
use chrono::{Local, Utc};

fn main() {
    let result = read_from_file_kirat(String::from("src/hello.txt"));
    match result {
        Ok(data) => println!("File contents: {}", data),
        Err(e) => println!("Error: {}", e),
    }
    println!("###########################################################################################");
    // Get the current date and time in UTC
    let now = Utc::now();
    println!("Current date and time in UTC: {}", now);

    // Format the date and time
    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted date and time: {}", formatted);

    // Get local time
    let local = Local::now();
    println!("Current date and time in local: {}", local);

    let mut s1 =String::from("Hello");
    let s2=s1.clone();
    s1.push_str("  Kaushik !!!!!");
    println!("{}",s1);
    println!("{}",s2);

    println!("$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$  References   $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$");

    let a1=String::from("Canada");
    do_something(&a1);
    println!("  {}  ",a1);
                 
}

fn do_something(a2:&String)
{
print!("{}",a2);
}
fn read_from_file_kirat(file_path: String) -> Result<String, String> {
    let result = read_to_string(file_path);
    match result {
        Ok(data) => Ok(data),
        Err(_err) => Err(String::from("Contents not found")),
    }
}
