use std::env;
use std::fs;
fn traverse_dir(path: &str, dot_folder: bool) {
    fs::read_dir(path).unwrap().for_each(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        if path.is_file() {
            println!("{:#}", path_str.replace("\\", "/"));
        } else {
            if path_str.contains(".") {
                if dot_folder {
                    traverse_dir(path_str, dot_folder);
                }
            } else {
                traverse_dir(path_str, dot_folder);
            }
        }
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let dot_folder = args.len() > 1 && args[1] == "-d";
    // let path = if args.len() > 1 { &args[1] } else { "." };
    traverse_dir("", dot_folder);
}
