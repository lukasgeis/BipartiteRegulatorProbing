#![deny(warnings)]

use std::{
    fs::{File, OpenOptions},
    io::{prelude::Write, BufReader},
    path::PathBuf,
};

use bpr::{
    model::{BipartiteRegulatorProbing, Instance},
    GoalType,
};
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
            BipartiteRegulatorProbing::init(BufReader::new(file))?
        }
        None => panic!("No input file was given!"),
    };

    let mut instance: Instance = bpr.create_instance();
    instance.optimal_solution(GoalType::MAX, opt.l);
    instance.optimal_solution(GoalType::SUM, opt.l);
    instance.namp(GoalType::MAX, opt.k, opt.l);
    instance.namp(GoalType::SUM, opt.k, opt.l);
    println!("{:?}", instance.get_results());
    

    let mut outfile = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(opt.log.unwrap())
        .unwrap();
    if let Err(e) = writeln!(outfile, "A new line!") {
        eprintln!("Couldn't write to file: {}", e);
    }

    Ok(())
}
