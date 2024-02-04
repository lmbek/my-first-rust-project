use std::fs;
use std::io;

mod hello_world_program;
mod read_write_files;

fn main() {
    // Hello World Program
    hello_world_program::hello_world_program::say_hello();

    // Read and Write Files
    read_write_files::read_write_files::write_and_read_with_modules();
    read_write_files::read_write_files::write_and_read_without_modules();


    // Only writing file (implemented in here)
    let filename = "hello_world.txt";
    let content = "Hello world!";

    match write_to_file(filename, content) {
        Ok(()) => println!("File written successfully!"),
        Err(err) => eprintln!("Error writing to file: {}", err),
    }
}

fn write_to_file(filename: &str, content: &str) -> io::Result<()> {
    fs::write(filename, content)?;
    Ok(())
}