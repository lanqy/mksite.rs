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
    let paths = fs::read_dir(config.source_dir).unwrap();
    for path in paths {
      
      let entry = path.unwrap();
      
      let entry_path = entry.path();
      
      let file_name = entry_path.file_name().unwrap();
      
      let file_name_as_str = file_name.to_str().unwrap();
      
      let file_name_as_string = String::from(file_name_as_str);
        let markdown_content = std::fs::read_to_string(&entry_path).unwrap();

        let front_matter_as_vec_str = parse_front_matter(&markdown_content);
        // let (matter, content) = matter::matter(&input).unwrap();
        
      println!("----- {:?}", front_matter_as_vec_str);
        // let html: String = markdown::to_html(&content);
        // let split = matter.split("\n");
        // let base_dir: String = config.target_dir.to_owned();
        // for s in split {
        //     if s.contains("created") {
        //         let vec: Vec<&str> = s.split(":").collect();
        //         let path = Path::join(Path::new(&vec[1].trim()),Path::new(&file_name_as_string));
        //         let dest = Path::join(Path::new(&base_dir), Path::new(&path));
        //         fs::create_dir_all(&dest).unwrap();
        //         let jfile_name = Path::new(&dest).join("index.html");
        //         let mut file = File::create(&jfile_name).unwrap();
        //         file.write_all(html.as_bytes()).unwrap();
        //     }
        // }
    }
}

pub fn parse_front_matter(contents: &str) -> Vec<&str> {
  let mut is_front_matter: bool = false;
  let mut counter_meet_delimiter: u8 = 0;
  let mut front_matter = Vec::new();

  for (line_number, line) in contents.lines().enumerate() {
      if (line_number == 0) & (line != "---") {
          // break the loop, if first line is not "---"
          break;
      } else if (line_number == 0) & (line == "---") {
          // if first line is "---", increase counter_meet_delimiter and set is_front_matter = true
          counter_meet_delimiter += 1;
          is_front_matter = true;
          continue;
      }

      if is_front_matter & (line == "---") {
          // if encounter the second delimiter "---", then break the loop and increase counter_meet_delimiter
          counter_meet_delimiter += 1;
          break;
      }

      if is_front_matter & ((line != "---") | (line != "")) {
          front_matter.push(line);
      }
  }

  if counter_meet_delimiter == 1 {
      // if there are not the closed delimiter
      front_matter = Vec::new();
  }

  front_matter
}
