use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    thread::sleep(Duration::from_secs(5));

    let message = "Hello, World!";
    println!("{}", message);

    let mut file = File::create("output.txt").expect("Failed to create file");
    file.write_all(message.as_bytes()).expect("Failed to write to file");
}