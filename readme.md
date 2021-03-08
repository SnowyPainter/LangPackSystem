# Lang Pack System
That help you make your international program.  
Or just variously people need.  

![./readme_resources/langpack.png](./readme_resources/langpack.png)

---
# Example
``` rust
fn main() {
    let filename = "./ko.lp";
    let split_code = ";";

    let title_position = 0;
    let lang_pack = lps::load_langpack(filename, split_code);

    let title_text = lps::lang_pack.get(&title_position).take();
    let mytitle = "My Title";
    let subtitle = "- and my sub title";

    match title_text {
        None => println!("Failed to load text this one"),
        Some(t) => {
            println!("{}",lps::to_form(t, &[mytitle, subtitle]));
        } 
    }
}
```
ko.lp
```
0;Title;$0 $1
```