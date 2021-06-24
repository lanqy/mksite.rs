/*
 * @Author: lanqy
 * @Date: 2021-06-20 09:49:36
 * @Description: a static site maker
 */
extern crate comrak;
extern crate markdown;
use comrak::{markdown_to_html, ComrakOptions};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str;
use serde_json::Result;
#[warn(unused_imports)]
#[derive(Serialize, Deserialize)]
pub struct Config {
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

pub struct Matter {
    title: String,
    created: String,
    description: String,
    author: String,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    title: String,
    created: String,
    link: String,
    description: String,
    content: String,
    author: String,
}

impl std::fmt::Display for Matter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "(title: {}, created: {}, description: {}, author: {})",
            self.title, self.created, self.description, self.author
        )
    }
}

impl std::fmt::Debug for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "title: {}, created: {}, description: {}, content: {}, author: {}, link: {}",
            self.title, self.created, self.description, self.content, self.author, self.link
        )
    }
}

fn main()-> Result<()> {
    let config_file = "./config.json";
    let data_file = "./data.json";
    let file_extension = ".md";
    let json_file_path = Path::new(config_file);
    let file = File::open(json_file_path).expect("file not found");
    let config: Config = serde_json::from_reader(file).expect("error while reading");
    let paths = fs::read_dir(&config.source_dir).unwrap();
    let base_dir: String = config.target_dir.to_owned();
    let mut posts:Vec<Post> = Vec::new();
    for path in paths {
        let entry = path.unwrap();
        let entry_path = entry.path();
        let file_name = entry_path.file_name().unwrap();
        let file_name_as_str = file_name.to_str().unwrap();
        let file_name_as_string = String::from(file_name_as_str).replace(file_extension, "");
        let markdown_content = std::fs::read_to_string(&entry_path).unwrap();
        let front_matter_as_vec_str = parse_front_matter(&markdown_content);
        let matter = make_matter(front_matter_as_vec_str);
        let post = create_post(
            &matter,
            &file_name_as_string,
            &markdown_content,
            &base_dir,
            &config,
        );
        posts.push(post);
    }
    let json = serde_json::to_string(&posts)?;
    let mut json_file = File::create(&data_file).unwrap();
    json_file.write_all(json.as_bytes()).unwrap();
    Ok(())
}

pub fn make_matter(matter: Vec<&str>) -> Matter {
    let mut matter_item = Matter {
        title: "".to_string(),
        created: "".to_string(),
        description: "".to_string(),
        author: "".to_string(),
    };

    for x in matter.iter() {
        if x.contains("created") {
            let vec: Vec<&str> = x.split(":").collect();
            matter_item.created = (&vec[1].trim()).to_string();
        }
        if x.contains("title") {
            let vec: Vec<&str> = x.split(":").collect();
            matter_item.title = (&vec[1].trim()).to_string();
        }
        if x.contains("description") {
            let vec: Vec<&str> = x.split(":").collect();
            matter_item.description = (&vec[1].trim()).to_string();
        }
        if x.contains("author") {
            let vec: Vec<&str> = x.split(":").collect();
            matter_item.author = (&vec[1].trim()).to_string();
        }
    }
    matter_item
}

pub fn create_post(
    matter: &Matter,
    file_name: &str,
    content: &str,
    base_dir: &str,
    config: &Config,
) -> Post {
    let mut options = ComrakOptions::default();
    options.extension.front_matter_delimiter = Some("---".to_owned());
    let path = Path::join(Path::new(&matter.created), Path::new(&file_name));
    let html = markdown_to_html(&content, &options);
    let dest = Path::join(Path::new(&base_dir), Path::new(&path));
    fs::create_dir_all(&dest).unwrap();
    let post_template_file = std::fs::read_to_string(&config.post_template_file).unwrap();
    let html_file = Path::new(&dest).join("index.html");
    let mut file = File::create(&html_file).unwrap();
    let mut post = Post {
        title: String::from(&matter.title),
        created: String::from(&matter.created),
        description: String::from(&matter.description),
        author: String::from(&matter.author),
        link: "".to_string(),
        content: "".to_string(),
    };

    post.content = html.as_str().to_string();
    post.link = path.as_path().display().to_string().replace("\\", "/");

    let htmls = post_template_file
        .replace("{{body}}", html.as_str())
        .replace("{{title}}", &matter.title)
        .replace("{{description}}", &matter.description);
    file.write_all(htmls.as_bytes()).unwrap();
    post
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
