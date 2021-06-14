extern crate markdown;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::str;
use walkdir::WalkDir;
#[warn(unused_imports)]
fn main() {
    for entry in WalkDir::new("./source") {
        let entry = entry.unwrap();
        if !entry.file_type().is_dir() {
            let path_string = entry.path().display().to_string();
            let file_name_ff = entry.path().file_name().unwrap().to_str().unwrap();
            let file_name = file_name_ff.replace(".md", "");
            println!("{}", file_name.replace(".md", ""));
            let input = std::fs::read_to_string(path_string).unwrap();
            let (matter, content) = matter::matter(&input).unwrap();
            let html: String = markdown::to_html(&content);
            let split = matter.split("\n");
            let mut base_dir: String = "./website/".to_owned();
            for s in split {
                if s.contains("created") {
                    let vec: Vec<&str> = s.split(":").collect();
                    base_dir.push_str(&vec[1].trim().to_string());
                    base_dir.push_str("/");
                    base_dir.push_str(&file_name);
                    println!("{}", base_dir);
                    fs::create_dir_all(&base_dir).unwrap();
                    base_dir.push_str("/index.html");
                    let mut file = File::create(&base_dir).unwrap();
                    file.write_all(html.as_bytes()).unwrap();
                } else {
                    if !Path::new(&base_dir).exists() {
                        base_dir.push_str("/");
                        base_dir.push_str(&file_name);
                        fs::create_dir_all(&base_dir).unwrap();
                        base_dir.push_str("/index.html");
                        let mut file = File::create(&base_dir).unwrap();
                        file.write_all(html.as_bytes()).unwrap();
                    }
                }
            }
        }
    }
}
