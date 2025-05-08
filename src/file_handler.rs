use std::fs::File;
use std::io::Read;
use std::path::Path;
use egui_file_dialog::FileDialog;

pub fn open_file(path: &Path) -> std::io::Result<String> {
    //let mut file = File::open("src/tests/file.json")?;
    let file = std::fs::read_to_string(path).unwrap();
    Ok(file)
}

pub fn get_file(path: &Path) -> String {
    let file_content = match open_file(path) {
        Ok(file_content) => file_content,
        Err(e) => {
            eprint!("THIS IS SO SAD: couldn't find file `{}", e);
            String::new()
        }
    };
    file_content
}

pub fn create_file() {
    println!("this function will create a file! :3");
}

pub fn save_file() {
    let mut file = File::create("src/tests/hola.txt");
    println!("saved file :D");
}