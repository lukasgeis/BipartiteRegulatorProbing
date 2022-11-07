#![deny(warnings)]

use std::{
    fs::{File, OpenOptions},
    io::{prelude::Write, BufReader},
    path::PathBuf,
    time::Instant,
};

use bpr::{
    model::{BipartiteRegulatorProbing, Instance},
    model_to_string, Algorithm, GoalType,
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

    #[structopt(long, parse(from_os_str))]
    input_time: Option<PathBuf>,

    #[structopt(short, long)]
    algorithm: Algorithm,

    #[structopt(short, default_value = "0")]
    k: usize,

    #[structopt(short, default_value = "0")]
    l: usize,

    #[structopt(long, default_value = "1")]
    iterations: usize,

    #[structopt(long)]
    bruteforce: bool,

    #[structopt(long, default_value = "0")]
    fraction: usize,

    #[structopt(long)]
    exclude_opt: bool,

    #[structopt(long)]
    coverage: bool,
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
            input_time.elapsed().as_secs_f64()
        ) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    let na: usize = bpr.get_na();

    for _ in 0..opt.iterations {
        let mut instance: Instance = bpr.create_instance();
        if opt.bruteforce {
            if !opt.exclude_opt {
                for l in 1..(na + 1) {
                    if opt.coverage {
                        panic!("Coverage is not implemented yet!");
                    }
                    instance.run_algorithm(GoalType::MAX, Algorithm::OPT, 0, l);
                    instance.run_algorithm(GoalType::SUM, Algorithm::OPT, 0, l);
                }
            }
            for k in 1..(na + 1) {
                for l in 1..(k + 1) {
                    if opt.coverage {
                        panic!("Coverage is not implemented yet!");
                    }

                    instance.run_algorithm(GoalType::MAX, opt.algorithm.clone(), k, l);
                    instance.run_algorithm(GoalType::SUM, opt.algorithm.clone(), k, l);
                }
            }
        } else if opt.fraction > 1 {
            for num_k in 1..(opt.fraction + 1) {
                let k: usize = na * (num_k as usize) / opt.fraction;
                for num_l in 1..(opt.fraction + 1) {
                    let l: usize = k * (num_l as usize) / opt.fraction;
                    if k > 0 && l > 0 {
                        if opt.coverage {
                            panic!("Coverage is not implemented yet!");
                        }
                        if !opt.exclude_opt {
                            instance.run_algorithm(GoalType::MAX, Algorithm::OPT, 0, l);
                            instance.run_algorithm(GoalType::SUM, Algorithm::OPT, 0, l);
                        }
                        instance.run_algorithm(GoalType::MAX, opt.algorithm.clone(), k, l);
                        instance.run_algorithm(GoalType::SUM, opt.algorithm.clone(), k, l);
                    }
                }
            }
        } else if opt.k >= 1 && opt.l >= 1 && opt.k >= opt.l {
            if opt.coverage {
                panic!("Coverage is not implemented yet!");
            }
            if !opt.exclude_opt {
                instance.run_algorithm(GoalType::MAX, Algorithm::OPT, 0, opt.l);
                instance.run_algorithm(GoalType::SUM, Algorithm::OPT, 0, opt.l);
            }
            instance.run_algorithm(GoalType::MAX, opt.algorithm.clone(), opt.k, opt.l);
            instance.run_algorithm(GoalType::SUM, opt.algorithm.clone(), opt.k, opt.l);
        }

        instance.log_results(&opt.log, None);
    }

    Ok(())
}
