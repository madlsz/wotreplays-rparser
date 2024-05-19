use clap::Parser;

use wotreplays_rparser::cl_args::Args;
use wotreplays_rparser::load_replay;

fn main() {
    let args = Args::parse();

    let path = "20240519_2045_poland-Pl21_CS_63_05_prohorovka.wotreplay";
    load_replay(path);

    // for byte in &buf {
    //     print!("{:02x} ", byte);
    // }
}
