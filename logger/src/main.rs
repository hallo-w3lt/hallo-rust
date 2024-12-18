use std::fs;
use std::io::Error;

fn extract_errors(source: &str) -> Vec<&str> {
    let mut errors = Vec::new();
    for line in source.lines() {
        if line.starts_with("ERROR") {
            errors.push(line);
        }
    }

    errors
}

fn handle_error() {
    // let content = fs::read_to_string("logs.txt").expect("Error reading file");
    // let errors = extract_errors(&content);
    // fs::write("errors.txt", errors.join("\n")).expect("Error writing file");

    match fs::read_to_string("logs.txt") {
        Ok(content) => {
            let errors = extract_errors(&content);
            match fs::write("errors.txt", errors.join("\n")) {
                Ok(_) => println!("Wrote errors to errors.txt file"),
                Err(error) => println!("Error: {}", error),
            }
        }
        Err(error) => println!("Error: {}", error),
    }
}

fn handle_error_with_try() -> Result<(), Error> {
    let content = fs::read_to_string("logs.txt")?;
    let errors = extract_errors(&content);
    fs::write("errors.txt", errors.join("\n"))?;
    Ok(())
}

fn main() {

    // handle_error();
    match handle_error_with_try() {
        Ok(_) => println!("Errors written to errors.txt"),
        Err(error) => println!("Error: {}", error),
    }
}
