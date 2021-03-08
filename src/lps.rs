use std::collections::HashMap;
use std::fs;
#[no_mangle]
pub fn to_form(text: &str, arguments: &[&str]) -> String {
    let mut text = text.to_owned();
    for i in 0..arguments.len() {
        let s = arguments[i];
        text = text.to_owned().replace(&format!("${0}", i),s);
    }
    text
}
#[no_mangle]
pub fn load_langpack(lpfile: &str, split: &str) -> HashMap<i32, String> {
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