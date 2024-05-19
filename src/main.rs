use bstr::ByteSlice;
use std::fs;

fn main() {
    let path = "20240519_2045_poland-Pl21_CS_63_05_prohorovka.wotreplay";
    let buf = fs::read(path).unwrap();

    let needle_start = "[{\"personal\":";
    let needle_end = "}}]";
    let idx_start = buf.find(needle_start).expect("not  found");
    let idx_end = buf[idx_start..].find(needle_end).expect("not  found");
    println!("start: {idx_start}");
    println!("end: {idx_end}");

    // for byte in &buf {
    //     print!("{:02x} ", byte);
    // }
}
