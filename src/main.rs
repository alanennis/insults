use std::fs::File;
use std::io::Read;
use rand::Rng;
use rand::rngs::ThreadRng;

fn lines_from_file(filename: &str) -> Vec<String> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let lines: Vec<String> = file_contents.split("\n")
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
    println!("{:?}", shake1.len());
    // println!("{}", file[9]);
    // let mut rng = rand::thread_rng();
    // println!("Integer: {}", rng.gen_range(0..file.len()));
    let line_rand = give_rand(shake1.len());
    println!("{}", shake1[line_rand]);

}
