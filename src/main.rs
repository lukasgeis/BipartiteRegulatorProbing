#![deny(warnings)]

use std::{fs::File, io::BufReader, path::PathBuf};

use bpr::model::{BipartiteRegulatorProbing, Instance};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "max-edge", about = "Max-Edge Variant of BPR")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    input: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    let bpr: BipartiteRegulatorProbing = match &opt.input {
        Some(path) => {
            let file = File::open(path)?;
            BipartiteRegulatorProbing::init(BufReader::new(file))?
        }
        None => panic!("No input file was given!"),
    };

    let instance: Instance = bpr.create_instance();
    println!("{:?}", instance.optimal_solution(bpr::GoalType::MAX, 2));
    println!("{:?}", instance.optimal_solution(bpr::GoalType::SUM, 2));
    println!("{:?}", &instance);

    Ok(())
}
