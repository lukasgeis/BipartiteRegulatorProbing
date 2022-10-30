#![deny(warnings)]

use std::{
    fs::{File, OpenOptions},
    io::{prelude::Write, BufReader},
    path::PathBuf,
    time::Instant,
};

use bpr::{
    model::{BipartiteRegulatorProbing, Instance},
    model_to_string,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "BipartiteRegulatorProbing",
    about = "Implementation of Algorithms for BPR"
)]
struct Opt {
    #[structopt(long, parse(from_os_str))]
    input: Option<PathBuf>,

    #[structopt(long, parse(from_os_str))]
    log: Option<PathBuf>,

    #[structopt(short, default_value = "1")]
    k: usize,

    #[structopt(short, default_value = "1")]
    l: usize,

    #[structopt(long, parse(from_os_str))]
    input_time: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    let input_time = Instant::now();

    // Create Model
    let mut bpr: BipartiteRegulatorProbing = match &opt.input {
        Some(path) => {
            let file = File::open(path)?;
            BipartiteRegulatorProbing::init(BufReader::new(file), true)?
        }
        None => panic!("No input file was given!"),
    };

    let input_duration = input_time.elapsed();
    if opt.input_time.is_some() {
        let mut time_outfile = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(opt.input_time.unwrap())
            .unwrap();
        if let Err(e) = writeln!(
            time_outfile,
            "{} -- Time: {:?}",
            model_to_string(&bpr),
            input_duration.as_secs_f64()
        ) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    // Create Instance
    let mut instance: Instance = bpr.create_instance();

    // Run Algorithms
    instance.run_algorithm(bpr::GoalType::MAX, bpr::Algorithm::OPT, opt.k, opt.l);
    instance.run_algorithm(bpr::GoalType::MAX, bpr::Algorithm::AMP, opt.k, opt.l);
    instance.run_algorithm(bpr::GoalType::MAX, bpr::Algorithm::NAMP, opt.k, opt.l);
    instance.run_algorithm(bpr::GoalType::MAX, bpr::Algorithm::SCG, opt.k, opt.l);
    instance.run_algorithm(bpr::GoalType::SUM, bpr::Algorithm::OPT, opt.k, opt.l);
    instance.run_algorithm(bpr::GoalType::SUM, bpr::Algorithm::AMP, opt.k, opt.l);
    instance.run_algorithm(bpr::GoalType::SUM, bpr::Algorithm::NAMP, opt.k, opt.l);
    instance.run_algorithm(bpr::GoalType::SUM, bpr::Algorithm::SCG, opt.k, opt.l);

    // Log Results to file
    instance.log_results(opt.log, None);

    Ok(())
}
