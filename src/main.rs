/*
 * @Author: lanqy
 * @Date: 2021-06-20 09:49:36
 * @Description: a static site maker
 */
extern crate comrak;
use comrak::{markdown_to_html, ComrakOptions};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str;
use std::{fs, io};
use xmlwriter::*;
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

fn main() -> Result<()> {
    let config_file = "./config.json";
    let data_file = "./data.json";
    let file_extension = ".md";
    let json_file_path = Path::new(config_file);
    let file = File::open(json_file_path).expect("file not found");
    let config: Config = serde_json::from_reader(file).expect("error while reading");
    let paths = fs::read_dir(&config.source_dir).unwrap();
    let source_vec: Vec<&str> = config.source_dir.split("/").collect();
    let page_paths = fs::read_dir(&source_vec[0].to_string()).unwrap();
    let page_paths_for_post = fs::read_dir(&source_vec[0].to_string()).unwrap();
    let base_dir: String = config.target_dir.to_owned();
    let static_dir: String = config.static_dir.to_owned();
    let mut posts: Vec<Post> = Vec::new();
    let mut nav_html_string = String::new();

    for page_path in page_paths {
        let page_path = page_path.unwrap();
        let page_entry_path = page_path.path();
        if !page_entry_path.is_dir() {
            let file_name = page_entry_path.file_name().unwrap();
            let file_name_as_str = file_name.to_str().unwrap();
            let file_name_as_string = String::from(file_name_as_str).replace(file_extension, "");
            let markdown_content = std::fs::read_to_string(&page_entry_path).unwrap();
            let path_file = Path::new("/").join(&file_name_as_string);
            let front_matter_as_vec_str = parse_front_matter(&markdown_content);
            let matter = make_matter(front_matter_as_vec_str);
            let nav_template_file = std::fs::read_to_string(&config.nav_template_file).unwrap();
            let nav_html = nav_template_file
                .replace("{{link}}", &path_file.to_str().unwrap())
                .replace("{{title}}", &matter.title);
            nav_html_string.push_str(&nav_html);
        }
    }

    for page_path_for_post in page_paths_for_post {
        let page_path = page_path_for_post.unwrap();
        let page_entry_path = page_path.path();
        if !page_entry_path.is_dir() {
            let file_name = page_entry_path.file_name().unwrap();
            let file_name_as_str = file_name.to_str().unwrap();
            let file_name_as_string = String::from(file_name_as_str).replace(file_extension, "");
            let markdown_content = std::fs::read_to_string(&page_entry_path).unwrap();
            let front_matter_as_vec_str = parse_front_matter(&markdown_content);
            let matter = make_matter(front_matter_as_vec_str);
            create_html(
                &matter,
                &file_name_as_string,
                &markdown_content,
                &base_dir,
                &config,
                &nav_html_string,
            );
        }
    }

    for path in paths {
        let entry = path.unwrap();
        let entry_path = entry.path();
        let file_name = entry_path.file_name().unwrap();
        let file_name_as_str = file_name.to_str().unwrap();
        let file_name_as_string = String::from(file_name_as_str).replace(file_extension, "");
        let markdown_content = std::fs::read_to_string(&entry_path).unwrap();
        let front_matter_as_vec_str = parse_front_matter(&markdown_content);
        let matter = make_matter(front_matter_as_vec_str);
        let post = create_html(
            &matter,
            &file_name_as_string,
            &markdown_content,
            &base_dir,
            &config,
            &nav_html_string,
        );
        posts.push(post);
    }
    posts.sort_by(|a, b| b.created.cmp(&a.created)); // sort by created field
    make_index_file(&posts, &config, &nav_html_string, &base_dir);
    let json = serde_json::to_string(&posts)?;
    let mut json_file = File::create(&data_file).unwrap();
    json_file.write_all(json.as_bytes()).unwrap();
    let _copy_dir = copy_dir_all(static_dir, base_dir); // copy static file in to website folder
    Ok(())
}

pub fn make_index_file(posts: &Vec<Post>, config: &Config, nav_html_string: &str, base_dir: &str) {
    let item_template_file = std::fs::read_to_string(&config.item_template_file).unwrap();
    let index_template_file = std::fs::read_to_string(&config.index_template_file).unwrap();
    let mut item_html_string = String::new();
    let index_file = "index.html".to_string();
    let dest = Path::join(Path::new(&base_dir), Path::new(&index_file));
    let mut file = File::create(&dest).unwrap();

    let opt = Options {
        use_single_quote: true,
        ..Options::default()
    };
    let mut w = XmlWriter::new(opt);
    w.write_declaration();
    w.start_element("feed");
    w.write_attribute("xmlns", "http://www.w3.org/2005/Atom");

    w.start_element("title");
    w.write_text(&config.site_name);
    w.end_element();

    w.start_element("summary");
    w.write_text(&config.site_name);
    w.end_element();

    w.start_element("link");
    w.write_attribute("href", &config.base_url);
    w.end_element();

    for post in posts.iter() {
        let item_html = item_template_file
            .replace("{{link}}", &post.link)
            .replace("{{created}}", &post.created)
            .replace("{{title}}", &post.title);
        item_html_string.push_str(&item_html);

        w.start_element("entry");
        w.start_element("title");
        w.write_text(&post.title);
        w.end_element();

        w.start_element("link");
        w.write_attribute("href", &post.link);
        w.end_element();

        w.start_element("summary");
        w.write_text(&post.description);
        w.end_element();

        w.start_element("author");
        w.write_text(&post.author);
        w.end_element();

        w.start_element("created");
        w.write_text(&post.created);
        w.end_element();

        w.end_element();
    }

    w.end_element();

    let mut atom_file = File::create("./atom.xml").unwrap();

    atom_file.write_all(w.end_document().as_bytes()).unwrap();

    let index_html = index_template_file
        .replace("{{siteName}}", &config.site_name)
        .replace("{{navs}}", &nav_html_string)
        .replace("{{lists}}", &item_html_string);

    file.write_all(index_html.as_bytes()).unwrap();
}

pub fn make_matter(matter: Vec<&str>) -> Matter {
    let mut matter_item = Matter {
        title: "".to_string(),
        created: "".to_string(),
        description: "".to_string(),
        author: "".to_string(),
    };

    for x in matter.iter() {
        let vec: Vec<&str> = x.split(":").collect();
        if x.contains("created") {
            matter_item.created = (&vec[1].trim()).to_string();
        }
        if x.contains("title") {
            matter_item.title = (&vec[1].trim()).to_string();
        }
        if x.contains("description") {
            matter_item.description = (&vec[1].trim()).to_string();
        }
        if x.contains("author") {
            matter_item.author = (&vec[1].trim()).to_string();
        }
    }
    matter_item
}

pub fn create_html(
    matter: &Matter,
    file_name: &str,
    content: &str,
    base_dir: &str,
    config: &Config,
    nav_html_string: &str,
) -> Post {
    let body_place_holder: String = "{{body}}".to_string();
    let title_place_holder: String = "{{title}}".to_string();
    let description_place_holder: String = "{{description}}".to_string();
    let navs_place_holder: String = "{{navs}}".to_string();
    let index_html = "index.html".to_string();
    let mut options = ComrakOptions::default();
    options.extension.front_matter_delimiter = Some("---".to_owned());
    let path = Path::join(Path::new(&matter.created), Path::new(&file_name));
    let html = markdown_to_html(&content, &options);
    let dest = Path::join(Path::new(&base_dir), Path::new(&path));
    fs::create_dir_all(&dest).unwrap();
    let post_template_file = std::fs::read_to_string(&config.post_template_file).unwrap();
    let html_file = Path::new(&dest).join(index_html);
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
        .replace(&body_place_holder, html.as_str())
        .replace(&title_place_holder, &matter.title)
        .replace(&description_place_holder, &matter.description)
        .replace(&navs_place_holder, &nav_html_string);
    file.write_all(htmls.as_bytes()).unwrap();
    post
}

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
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
