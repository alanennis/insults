use rand::Rng;
use std::fs::File;
use std::io::Read;

fn lines_from_file(filename: &str) -> Vec<String> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file {}", filename),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let lines: Vec<String> = file_contents
        .split("\n")
        .map(|s: &str| s.to_string())
        .collect();
    lines
}

fn give_rand(line_range: usize) -> usize {
    rand::thread_rng().gen_range(0..line_range)
}

fn main() {
    let file_name_1 = "./shake1.txt";
    let file_name_2 = "./shake2.txt";
    let file_name_3 = "./shake3.txt";

    let shake1 = lines_from_file(file_name_1);
    let shake2 = lines_from_file(file_name_2);
    let shake3 = lines_from_file(file_name_3);

    let line_rand_1 = give_rand(shake1.len() - 1);
    let line_rand_2 = give_rand(shake2.len() - 1);
    let line_rand_3 = give_rand(shake2.len() - 1);

    println!(
        "{} {} {}",
        shake1[line_rand_1], shake2[line_rand_2], shake3[line_rand_3]);
    
}
