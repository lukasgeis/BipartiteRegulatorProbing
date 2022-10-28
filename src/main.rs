#![deny(warnings)]

use std::{fs::File, io::BufReader, path::PathBuf};

use bpr::model::{BipartiteRegulatorProbing, Instance};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "max-edge", about = "Max-Edge Variant of BPR")]
struct Opt {
    #[structopt(long, parse(from_os_str))]
    input: Option<PathBuf>,

    #[structopt(long, parse(from_os_str))]
    log: Option<PathBuf>,

    #[structopt(long)]
    mdp: bool,

    #[structopt(short, long, default_value = "1")]
    l: usize,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    println!("{:?} -- {:?} -- {:?}", &opt.input, &opt.log, &opt.mdp);

    let bpr: BipartiteRegulatorProbing = match &opt.input {
        Some(path) => {
            let file = File::open(path)?;
            BipartiteRegulatorProbing::init(BufReader::new(file))?
        }
        None => panic!("No input file was given!"),
    };

    let instance: Instance = bpr.create_instance();
    println!("{:?}", instance.optimal_solution(bpr::GoalType::MAX, opt.l));
    println!("{:?}", instance.optimal_solution(bpr::GoalType::SUM, opt.l));
    println!("{:?}", &instance);

    Ok(())
}
