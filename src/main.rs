use std::{fs::OpenOptions, io::prelude::Write, path::PathBuf};

use bpr::{
    algorithms::Instance, bpr_to_string, model::BipartiteRegulatorProbing, solution_to_string,
    Algorithm, GoalFunction, Solution,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "BipartiteRegulatorProbing",
    about = "Run BPR on instances created at runtime"
)]
struct Opt {
    #[structopt(long, parse(from_os_str))]
    log: Option<PathBuf>,

    #[structopt(long, default_value = "1")]
    na: usize,

    #[structopt(long, default_value = "1")]
    nb: usize,

    #[structopt(long, default_value = "1")]
    vs: usize,

    #[structopt(long, default_value = "1")]
    iterations: usize,

    #[structopt(long, default_value = "1")]
    instances: usize,

    #[structopt(long, default_value = "MAX")]
    goal: GoalFunction,

    #[structopt(long, default_value = "OPT")]
    algorithm: Algorithm,

    #[structopt(long, default_value = "2")]
    parameters: usize,

    #[structopt(long)]
    not_opt: bool,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let mut outfile = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(opt.log.as_ref().unwrap())
        .unwrap();

    for _ in 0..opt.iterations {
        let mut bpr: BipartiteRegulatorProbing =
            BipartiteRegulatorProbing::new(opt.na, opt.nb, opt.vs);
        let bpr_output: String = bpr_to_string(&bpr);
        for _ in 0..opt.instances {
            let mut instance: Instance = bpr.create_instance();
            for num_k in 1..opt.parameters {
                let k: usize = num_k * opt.na / opt.parameters;
                for num_l in 1..=opt.parameters {
                    let l: usize = num_l * k / opt.parameters;
                    if k > 0 && l > 0 {
                        if !opt.not_opt && num_k == 1 && opt.algorithm != Algorithm::OPT {
                            let all_solutions: Vec<Solution> =
                                instance.run_algorithm(opt.goal.clone(), Algorithm::OPT, k, l);
                            if all_solutions.len() > 0 {
                                for sol in all_solutions {
                                    if let Err(e) = writeln!(
                                        outfile,
                                        "{} -- {} -- Code: {}",
                                        bpr_output,
                                        solution_to_string(&sol),
                                        instance.get_coding()
                                    ) {
                                        eprintln!("Could not write to file: {}", e);
                                    }
                                }
                            }
                        }
                        let all_solutions: Vec<Solution> =
                            instance.run_algorithm(opt.goal.clone(), opt.algorithm.clone(), k, l);
                        if all_solutions.len() > 0 {
                            for sol in all_solutions {
                                if let Err(e) = writeln!(
                                    outfile,
                                    "{} -- {} -- Code: {}",
                                    bpr_output,
                                    solution_to_string(&sol),
                                    instance.get_coding()
                                ) {
                                    eprintln!("Could not write to file: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
