#![deny(warnings)]

use std::path::PathBuf;

use structopt::StructOpt;

use bpr::{distributions::Distribution};

#[derive(Debug, StructOpt)]
#[structopt(name = "max-edge", about = "Max-Edge Variant of BPR")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    input: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", &opt.input);

    let x: Distribution = Distribution::from_list(&vec![0.12, 0.38, 0.24, 0.13, 0.07, 0.06]);

    for _ in 0..25 {
        println!("{:?}", x.draw_value());
    }
}
