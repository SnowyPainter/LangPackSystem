use std::collections::HashMap;
use std::fs;

fn to_form(text: &str, arguments: &[&str]) -> String {
    let mut text = text.to_owned();
    for i in 0..arguments.len() {
        let s = arguments[i];
        text = text.to_owned().replace(&format!("${0}", i),s);
    }
    text
}



fn load_langpack(lpfile: &str, split: &str) -> HashMap<i32, String> {
    let key_position = 0;
    let text_position = 2;

    let lines = fs::read_to_string(lpfile).ok().unwrap();
    let mut lang_pack: HashMap<i32, String> = HashMap::new();

    for line in lines.split("\n") {
        let splited:Vec<&str> = line.split(split).collect();
        let key = splited[key_position].parse();
        match key {
            Err(_) => continue,
            Ok(k) => {
                let text = splited[text_position];
                lang_pack.insert(k, text.to_owned());
            }
        }
    }
    lang_pack
}

fn main() {
    let filename = "./ko.lp";
    let split_code = ";";

    let title_position = 0;
    let lang_pack = load_langpack(filename, split_code);

    let title_text = lang_pack.get(&title_position).take();
    let mytitle = "My Title";
    let subtitle = "- and my sub title";

    match title_text {
        None => println!("Failed to load text this one"),
        Some(t) => {
            println!("{}",to_form(t, &[mytitle, subtitle]));
        } 
    }
}