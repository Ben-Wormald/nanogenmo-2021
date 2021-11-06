use std::fs::{File, metadata};
use std::io::{prelude::*, BufReader, BufWriter};

#[allow(dead_code)]
const SOURCE: &str = "./text/paradise-lost.txt";
#[allow(dead_code)]
const TEST: &str = "./text/test.txt";
const OUTPUT: &str = "./text/out.txt";

fn main() {
    let source = TEST;
    let total_bytes = metadata(&source).unwrap().len();

    let source = File::open(source).expect("couldn't read source");
    let reader = BufReader::new(source);

    let output = File::create(OUTPUT).expect("couldn't create output");
    let mut writer = BufWriter::new(output);

    let mut bytes_read = 0;

    for line in reader.lines() {
        let line = process_line(&line.unwrap(), &bytes_read, &total_bytes);
        bytes_read += line.len() as u64 + 1;
        writer.write(&line).unwrap();
        writer.write("\n".as_bytes()).unwrap();
    }
}

fn process_line(line: &str, bytes_read: &u64, total_bytes: &u64) -> Vec<u8> {
    let bytes = line.bytes();
    let offset = *bytes_read as f64 / *total_bytes as f64;
    let offset = offset.round() as u8;
    bytes.map(|byte| byte + offset).collect()
}
