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

    #[structopt(short, default_value = "1")]
    k: usize,

    #[structopt(short, default_value = "1")]
    l: usize,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    let bpr: BipartiteRegulatorProbing = match &opt.input {
        Some(path) => {
            let file = File::open(path)?;
            BipartiteRegulatorProbing::init(BufReader::new(file), true)?
        }
        None => panic!("No input file was given!"),
    };

    let mut instance: Instance = bpr.create_instance();
    instance.run_algorithm(bpr::GoalType::MAX, bpr::Algorithm::OPT, opt.k, opt.l);
    instance.run_algorithm(bpr::GoalType::SUM, bpr::Algorithm::OPT, opt.k, opt.l);
    instance.log_results(opt.log, None);

    Ok(())
}
