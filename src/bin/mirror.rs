use std::{env, fs};
use text_util::text_mirror::mirror_lines;
fn main() {
    let args: Vec<String> = env::args().collect();
    let Some(file) = args.get(1) else {
        println!("File name required!");
        return;
    };

    let Ok(file) = fs::read_to_string(file) else {
        println!("Failed to open {}", file);
        return;
    };

    let lines: Vec<String> = file.lines().map(|s| s.to_string()).collect();
    let mirrored_lines: Vec<String> = mirror_lines(&lines);

    for (original, mirrored) in lines.iter().zip(mirrored_lines.iter()) {
        println!("{}  |  {}", original, mirrored);
    }
}
