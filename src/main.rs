use std::{
    fs::{File, OpenOptions},
    io::{prelude::Write, BufReader},
    path::PathBuf,
};

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
    file: Option<PathBuf>,

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
    poisson: bool,

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
        let mut bpr: BipartiteRegulatorProbing = match &opt.file {
            Some(path) => {
                let file = File::open(path).expect("Could not find file!");
                BipartiteRegulatorProbing::init(BufReader::new(file))
                    .expect("Could not parse File!")
            }
            None => BipartiteRegulatorProbing::new(opt.na, opt.nb, opt.vs, opt.poisson),
        };

        let na: usize = bpr.get_na();

        let bpr_output: String = bpr_to_string(&bpr);
        for _ in 0..opt.instances {
            let mut instance: Instance = bpr.create_instance();
            // Run OPT on all possible l values
            if !opt.not_opt {
                let mut l_values_run: Vec<usize> = Vec::new();
                for opt_k in 1..opt.parameters {
                    let k: usize = opt_k * na / opt.parameters;
                    for opt_l in 1..=opt.parameters {
                        let l: usize = opt_l * k / opt.parameters;
                        if k > 0 && l > 0 && !l_values_run.contains(&l) {
                            l_values_run.push(l);
                            let all_solutions: Vec<Solution> =
                                instance.run_algorithm(opt.goal.clone(), Algorithm::OPT, 0, l);
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
            // Run the specified algorithms
            for num_k in 1..opt.parameters {
                let k: usize = num_k * na / opt.parameters;
                for num_l in 1..=opt.parameters {
                    let l: usize = num_l * k / opt.parameters;
                    if k > 0 && l > 0 {
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
