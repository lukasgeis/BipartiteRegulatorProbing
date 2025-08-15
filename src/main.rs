use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    time::Instant,
};

use bpr::{
    compute_k_l_pairs, compute_opt_l_values,
    model::{BipartiteRegulatorProbing, ProbeMax},
    GoalFunction,
};

use rayon::prelude::*;
use serde_derive::Serialize;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "BipartiteRegulatorProbing",
    about = "Evaluate algorithms for BPR"
)]
struct Parameters {
    #[structopt(long, parse(from_os_str))]
    log: Option<PathBuf>,

    #[structopt(long, default_value = "16")]
    na: usize,

    #[structopt(long, default_value = "16")]
    nb: usize,

    #[structopt(long, default_value = "2")]
    vs: usize,

    #[structopt(long, default_value = "1")]
    iterations: usize,

    #[structopt(long, default_value = "1")]
    instances: usize,

    #[structopt(long, default_value = "MAX")]
    goal: GoalFunction,

    #[structopt(long)]
    poisson: bool,

    /// Use the IP-Formulation instead when computing Opt (for Coverage only)
    #[structopt(long)]
    ipopt: bool,
}

#[derive(Serialize)]
struct Result {
    na: usize,
    nb: usize,
    vs: usize,
    goal: String,
    algo: String,
    k: usize,
    l: usize,
    val: usize,
    ins_id: usize,
    iter_id: usize,
    time: f64,
}

fn main() -> std::io::Result<()> {
    let params = Parameters::from_args();

    assert!(params.log.is_some(), "Log Path must be given!");

    std::fs::create_dir_all(params.log.as_ref().unwrap())?;

    let logfiles: Vec<File> = (0..params.iterations)
        .map(|i| {
            let mut path = params.log.as_ref().unwrap().clone().into_os_string();
            path.push(format!("/{:?}_{}.json", params.goal, i));

            OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .unwrap()
        })
        .collect();

    println!("{}", num_cpus::get());

    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build_global()
        .unwrap_or_else(|e| println!("Failed to set the number of threads used by rayon: {}", e));

    match params.goal {
        GoalFunction::MAX => eval_max(&params, logfiles),
        GoalFunction::SUM => eval_sum(&params, logfiles),
        GoalFunction::COV => eval_cov(&params, logfiles),
    };

    Ok(())
}

fn eval_max(params: &Parameters, logfiles: Vec<File>) {
    logfiles
        .into_par_iter()
        .enumerate()
        .for_each(|(i, mut logfile)| {
            let rng = &mut rand::rng();
            let pm = ProbeMax::from_bpr_max(&BipartiteRegulatorProbing::create_random(
                rng,
                params.na,
                params.nb,
                params.vs,
                params.poisson,
                params.instances,
            ));

            for j in 0..params.instances {
                let ins = pm.create_instance(j);

                for opt_l in compute_opt_l_values(params.na) {
                    let res = Result {
                        na: params.na,
                        nb: params.nb,
                        vs: params.vs,
                        goal: "MAX".to_owned(),
                        algo: "OPT".to_owned(),
                        k: params.na,
                        l: opt_l,
                        val: ins.get_optimal_value(opt_l),
                        ins_id: i,
                        iter_id: j,
                        time: ins.get_optimal_time(),
                    };
                    let _ = writeln!(logfile, "{}", serde_json::to_string(&res).unwrap());
                }

                for (alg_k, alg_l) in compute_k_l_pairs(params.na) {
                    let (amp_val, amp_time) = ins.adaptive_policy(alg_k, alg_l);
                    let res = Result {
                        na: params.na,
                        nb: params.nb,
                        vs: params.vs,
                        goal: "MAX".to_owned(),
                        algo: "AMP".to_owned(),
                        k: alg_k,
                        l: alg_l,
                        val: amp_val,
                        ins_id: i,
                        iter_id: j,
                        time: amp_time,
                    };
                    let _ = writeln!(logfile, "{}", serde_json::to_string(&res).unwrap());

                    let (namp_val, namp_time) = ins.get_non_adap_value(alg_k, alg_l);
                    let res = Result {
                        na: params.na,
                        nb: params.nb,
                        vs: params.vs,
                        goal: "MAX".to_owned(),
                        algo: "NAMP".to_owned(),
                        k: alg_k,
                        l: alg_l,
                        val: namp_val,
                        ins_id: i,
                        iter_id: j,
                        time: namp_time,
                    };
                    let _ = writeln!(logfile, "{}", serde_json::to_string(&res).unwrap());
                }
            }
        });
}

fn eval_sum(params: &Parameters, logfiles: Vec<File>) {
    logfiles
        .into_par_iter()
        .enumerate()
        .for_each(|(i, mut logfile)| {
            let rng = &mut rand::rng();
            let pm = ProbeMax::from_bpr_sum(&BipartiteRegulatorProbing::create_random(
                rng,
                params.na,
                params.nb,
                params.vs,
                params.poisson,
                params.instances,
            ));

            for j in 0..params.instances {
                let ins = pm.create_instance(j);

                for opt_l in compute_opt_l_values(params.na) {
                    let res = Result {
                        na: params.na,
                        nb: params.nb,
                        vs: params.vs,
                        goal: "SUM".to_owned(),
                        algo: "OPT".to_owned(),
                        k: params.na,
                        l: opt_l,
                        val: ins.get_optimal_value(opt_l),
                        ins_id: i,
                        iter_id: j,
                        time: ins.get_optimal_time(),
                    };
                    let _ = writeln!(logfile, "{}", serde_json::to_string(&res).unwrap());
                }

                for (alg_k, alg_l) in compute_k_l_pairs(params.na) {
                    let (amp_val, amp_time) = ins.adaptive_policy(alg_k, alg_l);
                    let res = Result {
                        na: params.na,
                        nb: params.nb,
                        vs: params.vs,
                        goal: "SUM".to_owned(),
                        algo: "AMP".to_owned(),
                        k: alg_k,
                        l: alg_l,
                        val: amp_val,
                        ins_id: i,
                        iter_id: j,
                        time: amp_time,
                    };
                    let _ = writeln!(logfile, "{}", serde_json::to_string(&res).unwrap());

                    let (namp_val, namp_time) = ins.get_non_adap_value(alg_k, alg_l);
                    let res = Result {
                        na: params.na,
                        nb: params.nb,
                        vs: params.vs,
                        goal: "SUM".to_owned(),
                        algo: "NAMP".to_owned(),
                        k: alg_k,
                        l: alg_l,
                        val: namp_val,
                        ins_id: i,
                        iter_id: j,
                        time: namp_time,
                    };
                    let _ = writeln!(logfile, "{}", serde_json::to_string(&res).unwrap());
                }
            }
        });
}

fn eval_cov(params: &Parameters, logfiles: Vec<File>) {
    logfiles
        .into_par_iter()
        .enumerate()
        .for_each(|(i, mut logfile)| {
            let rng = &mut rand::rng();
            let mut bpr = BipartiteRegulatorProbing::create_random(
                rng,
                params.na,
                params.nb,
                params.vs,
                params.poisson,
                params.instances,
            );

            for (k, l) in compute_k_l_pairs(params.na) {
                bpr.compute_namp_cov_policy(k, l);
            }

            for j in 0..params.instances {
                let ins = bpr.create_instance(j);

                for opt_l in compute_opt_l_values(params.na) {
                    let res = if params.ipopt {
                        let timer = Instant::now();
                        let (ip_val, _) = bpr::ip::solve_cov_instance(params.na, params.nb, opt_l, &ins.realizations);
                        let ip_time = timer.elapsed().as_secs_f64();

                        Result{
                            na: params.na,
                            nb: params.nb,
                            vs: params.vs,
                            goal: "COV".to_owned(),
                            algo: "OPT".to_owned(),
                            k: params.na,
                            l: opt_l,
                            val: ip_val,
                            ins_id: i,
                            iter_id: j,
                            time: ip_time,
                        }
                    } else {
                        Result{
                            na: params.na,
                            nb: params.nb,
                            vs: params.vs,
                            goal: "COV".to_owned(),
                            algo: "OPT".to_owned(),
                            k: params.na,
                            l: opt_l,
                            val: ins.get_opt_cov_value(opt_l),
                            ins_id: i,
                            iter_id: j,
                            time: ins.get_opt_cov_time(),
                        }
                    };
                    let _ = writeln!(logfile, "{}", serde_json::to_string(&res).unwrap());
                }

                for (alg_k, alg_l) in compute_k_l_pairs(params.na) {
                    let (amp_val, amp_time) = ins.adaptive_policy(alg_k, alg_l);
                    let res = Result {
                        na: params.na,
                        nb: params.nb,
                        vs: params.vs,
                        goal: "COV".to_owned(),
                        algo: "AMP".to_owned(),
                        k: alg_k,
                        l: alg_l,
                        val: amp_val,
                        ins_id: i,
                        iter_id: j,
                        time: amp_time,
                    };
                    let _ = writeln!(logfile, "{}", serde_json::to_string(&res).unwrap());

                    let timer = Instant::now();
                    let namp_val = ins.eval_policy(bpr.get_policy(alg_k, alg_l).unwrap(), alg_l);
                    let res = Result {
                        na: params.na,
                        nb: params.nb,
                        vs: params.vs,
                        goal: "COV".to_owned(),
                        algo: "NAMP".to_owned(),
                        k: alg_k,
                        l: alg_l,
                        val: namp_val,
                        ins_id: i,
                        iter_id: j,
                        time: bpr.get_policy_time(alg_k, alg_l).unwrap()
                            + timer.elapsed().as_secs_f64(),
                    };
                    let _ = writeln!(logfile, "{}", serde_json::to_string(&res).unwrap());

                    let timer = Instant::now();
                    let ext_val = ins.eval_policy(bpr.get_policy(alg_k, alg_k).unwrap(), alg_l);
                    let res = Result {
                        na: params.na,
                        nb: params.nb,
                        vs: params.vs,
                        goal: "COV".to_owned(),
                        algo: "EXT".to_owned(),
                        k: alg_k,
                        l: alg_l,
                        val: ext_val,
                        ins_id: i,
                        iter_id: j,
                        time: bpr.get_policy_time(alg_k, alg_k).unwrap()
                            + timer.elapsed().as_secs_f64(),
                    };
                    let _ = writeln!(logfile, "{}", serde_json::to_string(&res).unwrap());
                }
            }
        });
}
