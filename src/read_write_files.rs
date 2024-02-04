pub mod read_write_files {
    use std::fs;
    use std::io;

    pub fn write_and_read_with_modules() {
        let filename = "file_with_modules.txt";
        let content_to_write = "Hello from a file with modules!";

        // Write to file
        match write_to_file(filename, content_to_write) {
            Ok(()) => println!("File written successfully!"),
            Err(err) => {
                eprintln!("Error writing to file: {}", err);
                return;
            }
        }

        // Read from file
        match read_from_file(filename) {
            Ok(read_content) => {
                println!("File content with modules:");
                println!("{}", read_content);
            }
            Err(err) => {
                eprintln!("Error reading from file: {}", err);
                return;
            }
        }

        // Clean up: Remove the file after reading
        if let Err(err) = fs::remove_file(filename) {
            eprintln!("Error removing file: {}", err);
            return;
        }
        println!("File removed successfully!");
    }

    pub fn write_and_read_without_modules() {
        let filename = "file_without_modules.txt";
        let content_to_write = "Hello from a file without modules!";

        // Write to file
        match write_to_file(filename, content_to_write) {
            Ok(()) => println!("File written successfully!"),
            Err(err) => {
                eprintln!("Error writing to file: {}", err);
                return;
            }
        }

        // Read from file
        match read_from_file(filename) {
            Ok(read_content) => {
                println!("File content without modules:");
                println!("{}", read_content);
            }
            Err(err) => {
                eprintln!("Error reading from file: {}", err);
                return;
            }
        }

        // Clean up: Remove the file after reading
        if let Err(err) = fs::remove_file(filename) {
            eprintln!("Error removing file: {}", err);
            return;
        }
        println!("File removed successfully!");
    }

    fn write_to_file(filename: &str, content: &str) -> io::Result<()> {
        fs::write(filename, content)?;
        Ok(())
    }

    fn read_from_file(filename: &str) -> io::Result<String> {
        let content = fs::read_to_string(filename)?;
        Ok(content)
    }
}
