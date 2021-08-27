use std::io;
use std::fs;
use std::string;
use std::process::Command;

fn convert_191(){
    for element in std::path::Path::new(r"C:/Users/1/Downloads/NAP_191").read_dir().unwrap() {
        let path = element.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "doc" || extension == "docx" {
                fs::remove_file(path);
            }
        }
    }
}

fn main() {
    let mut x = String::new();
    println! ("Избери НАП");
    println! ("191: 1");
    println! ("74: 2");
    io::stdin().read_line(&mut x).expect("Failed to read line");
    let y: u8 = x.trim().parse().expect("Не е номер");
    if y == 1 {
        Command::new("cmd")
                .args(&["/C", "D:/Rust/Nap_convert/Doc2Pdf/bulk-convert-Word2PDF_NAP_191.bat"])
                .output()
                .expect("failed to execute process");
        convert_191();
    } else if y == 2{
        Command::new("cmd")
                .args(&["/C", "D:/Rust/Nap_convert/Doc2Pdf/bulk-convert-Word2PDF_NAP_74.bat"])
                .output()
                .expect("failed to execute process");

    }
}
