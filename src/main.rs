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

fn main() {
    let filename = "./ko.lp";
    let split_code = ";";

    let title_position = 0;

    let key_position = 0;
    let text_position = 2;

    let lines = fs::read_to_string(filename)
        .expect(&format!("Failed to read {}", filename));

    let lines = lines.split("\n");

    let mut lang_pack = HashMap::new();

    for line in lines {
        let splited:Vec<&str> = line.split(split_code).collect();
        let key:i32 = splited[key_position].parse().ok().unwrap();
        let text:&str = splited[text_position];

        lang_pack.insert(key, text);
    }

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