#![deny(warnings)]

use std::path::PathBuf;

use structopt::StructOpt;

use bpr::helper::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "max-edge", about = "Max-Edge Variant of BPR")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    input: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", &opt.input);

    let t: Vec<f64> = vec![0.1, 0.2];
    println!("{:?}", &t);

    println!("{:?}", t.into_iter().sum::<f64>() - 0.3);

    println!("{:?}", boolean_combination(5));
}
