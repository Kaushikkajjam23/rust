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

}

fn read_from_file_kirat(file_path: String) -> Result<String, String> {
    let result = read_to_string(file_path);
    match result {
        Ok(data) => Ok(data),
        Err(_err) => Err(String::from("Contents not found")),
    }
}
