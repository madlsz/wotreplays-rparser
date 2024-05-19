use bstr::ByteSlice;
use clap::Parser;
use serde_json::{Result, Value};
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    /// list of replays to parse
    #[arg(required = true)]
    files: Vec<String>,
}

struct Player {
    id: u32,
}

fn main() {
    let args = Args::parse();

    let path = "20240519_2045_poland-Pl21_CS_63_05_prohorovka.wotreplay";
    let buf = fs::read(path).unwrap();

    let needle_start = "[{\"personal\":";
    let needle_end = "}}]";
    let idx_start = buf.find(needle_start).expect("not  found");
    let idx_end = buf.find_iter(needle_end).last().expect("not  found");
    println!("start: {idx_start}");
    println!("end: {idx_end}");

    let buf_1 = String::from_utf8(buf[idx_start..idx_end + needle_end.len()].to_vec()).unwrap();

    let v: Value = serde_json::from_str(&buf_1).unwrap();

    println!("{}", v[0]);

    // for byte in &buf {
    //     print!("{:02x} ", byte);
    // }
}
