use std::fs;
use std::process::Command;


fn main() {
    Command::new("cmd")
            .args(&["/C", "D:/Rust/Nap_convert/bulk-convert-Word2PDF.bat"])
            .output()
            .expect("failed to execute process");

    for element in std::path::Path::new(r"C:/Users/1/Downloads/NAP_191").read_dir().unwrap() {
        let path = element.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "doc" || extension == "docx" {
                fs::remove_file(path);
            }
        }
    }
}
