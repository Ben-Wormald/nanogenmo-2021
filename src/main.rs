use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter};

#[allow(dead_code)]
const SOURCE: &str = "./text/paradise-lost.txt";
#[allow(dead_code)]
const TEST: &str = "./text/test.txt";
const OUTPUT: &str = "./text/out.txt";

fn main() {
    let source = SOURCE;
    let source = File::open(source).expect("couldn't read source");
    let reader = BufReader::new(source);

    let output = File::create(OUTPUT).expect("couldn't create output");
    let mut writer = BufWriter::new(output);

    for line in reader.lines() {
        let line = process_line(&line.unwrap());
        writer.write(line.as_bytes()).unwrap();
        writer.write("\n".as_bytes()).unwrap();
    }
}

fn process_line(line: &str) -> String {
    let bytes = line.bytes();
    let bytes = bytes.map(|byte| byte + 1).collect();
    String::from_utf8(bytes).unwrap()
}
