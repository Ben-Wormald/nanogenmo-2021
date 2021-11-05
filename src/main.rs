use std::fs::File;
use std::io::{prelude::*, BufReader};

#[allow(dead_code)]
const SOURCE: &str = "./text/paradise-lost.txt";
#[allow(dead_code)]
const TEST: &str = "./text/test.txt";

fn main() {
    let source = File::open(TEST).unwrap();
    let reader = BufReader::new(source);

    for line in reader.lines() {
        let line = process_line(&line.unwrap());
        println!("{}", line);
    }
}

fn process_line(line: &str) -> String {
    let bytes = line.bytes();
    let bytes = bytes.map(|byte| byte + 1).collect();
    String::from_utf8(bytes).unwrap()
}
