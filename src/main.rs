/*
 * @Author: lanqy
 * @Date: 2021-06-20 09:49:36
 * @Description: file content
 */
extern crate markdown;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::str;
use walkdir::WalkDir;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::BufReader;
use std::io;
#[warn(unused_imports)]

#[derive(Serialize, Deserialize)]
struct Config {
    site_name: String,
    static_dir: String,
    base_url: String,
    source_dir: String,
    target_dir: String,
    nav_template_file: String,
    post_template_file: String,
    index_template_file: String,
    item_template_file: String,
}

fn main() {
    let json_file_path = Path::new("./config.json");
    let file = File::open(json_file_path).expect("file not found");
    let config: Config = serde_json::from_reader(file).expect("error while reading");

   println!("site_name: {}", config.site_name);
   println!("static_dir: {}", config.static_dir);
   println!("base_url: {}", config.base_url);
   println!("source_dir: {}", config.source_dir);
   println!("target_dir: {}", config.target_dir);
   println!("nav_template_file: {}", config.nav_template_file);
   println!("post_template_file: {}", config.post_template_file);
   println!("item_template_file: {}", config.item_template_file);
   fs::remove_dir_all(&config.target_dir);
    for entry in WalkDir::new(config.source_dir) {
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
            let mut base_dir: String = config.target_dir.to_owned();
            let mut dest = Path::join(Path::new(&base_dir), Path::new("ViGEmClient.dll"));
            for s in split {
                if s.contains("created") {
                    let vec: Vec<&str> = s.split(":").collect();
                    // base_dir.push_str(&vec[1].trim().to_string());
                    let path = Path::join(Path::new(&vec[1].trim()),Path::new(&file_name));
                    let dest = Path::join(Path::new(&base_dir), Path::new(&path));
                    println!("{:?}", dest);
                    // base_dir.push_str("/");
                    // base_dir.push_str(&file_name);
                    // println!("{}", base_dir);
                    fs::create_dir_all(&dest).unwrap();
                    // base_dir.push_str("/index.html");
                    let jfile_name = Path::join(&dest, Path::new("/index.html"));
                    
                    println!("{:?}", jfile_name);
                    let mut file = File::create(&jfile_name).unwrap();
                    file.write_all(html.as_bytes()).unwrap();
                } else {
                    if !Path::new(&dest).exists() {
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
    fs::copy(config.static_dir, config.target_dir);
}
