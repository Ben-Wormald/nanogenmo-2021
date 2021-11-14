use std::env;
use std::fs::{File, metadata};
use std::io::{prelude::*, BufReader, BufWriter};
use rand::prelude::*;

const OUTPUT: &str = "./text/with-loss-of-eden.txt";

const BITS: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
const DECAY_POWER: f64 = 7.0;

fn main() {
    let args: Vec<String> = env::args().collect();

    let total_bytes = &args[1..].iter().fold(0, |bytes, source|
        bytes + metadata(source).unwrap().len()
    );

    let mut bytes_read = 0;

    let output = File::create(OUTPUT).expect("couldn't create output");
    let mut writer = BufWriter::new(output);

    let mut rng = thread_rng();

    args[1..].iter().for_each(|source|
        process_source(source, &mut bytes_read, &total_bytes, &mut writer, &mut rng)
    );
}

fn process_source(
    source: &str,
    bytes_read: &mut u64,
    total_bytes: &u64,
    writer: &mut BufWriter<File>,
    rng: &mut ThreadRng,
) {
    let source = File::open(source).expect(&format!("couldn't read source {}", source));
    let reader = BufReader::new(source);

    for line in reader.lines() {
        let line = process_line(&line.unwrap(), &bytes_read, &total_bytes, rng);
        *bytes_read += line.len() as u64 + 1;
        writer.write(&line).unwrap();
        writer.write("\n".as_bytes()).unwrap();
    }
    writer.write("\n".as_bytes()).unwrap();
}

fn process_line(
    line: &str,
    bytes_read: &u64,
    total_bytes: &u64,
    rng: &mut ThreadRng,
) -> Vec<u8> {
    let bytes = line.bytes();
    let chance = *bytes_read as f64 / *total_bytes as f64;
    let chance = chance.powf(DECAY_POWER);

    bytes.map(|byte|
        BITS.iter().fold(byte, |byte, bit| {
            let should_flip: f64 = rng.gen();
            if should_flip < chance { byte ^ bit } else { byte }
        })
    ).collect()
}
