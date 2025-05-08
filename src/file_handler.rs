use std::fs::File;
use std::io::Read;

pub fn open_file() -> std::io::Result<String> {
    let mut file = File::open("src/tests/file.json")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn get_file() -> String {
    let file_content = match open_file() {
        Ok(file_content) => file_content,
        Err(e) => {
            eprint!("THIS IS SO SAD: couldn't find file {}", e);
            String::new()
        }
    };
    file_content
}

pub fn create_file() {
    println!("this function will create a file! :3");
}

pub fn save_file() {
    println!("this function will save a file! :3");
}