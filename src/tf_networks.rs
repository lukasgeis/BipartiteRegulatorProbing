use std::{fs::File, io::{BufRead, BufReader, Error, ErrorKind}, path::PathBuf};

use bpr::{distributions::WeightedDistribution, model::BipartiteRegulatorProbing, GoalFunction};
use statrs::distribution::{Binomial, Discrete, Poisson};
use structopt::StructOpt;



#[derive(Debug, StructOpt)]
struct Parameters {
    #[structopt(long, parse(from_os_str))]
    file: PathBuf,

    #[structopt(long, parse(from_os_str))]
    log: PathBuf,

    #[structopt(short = "i", long, default_value = "1")]
    iterations: usize,

    #[structopt(short = "k", default_value = "10")]
    k: usize,

    #[structopt(short = "l", default_value = "5")]
    l: usize,
}


fn main() -> std::io::Result<()> {
    let params = Parameters::from_args();

    let _ = std::fs::create_dir_all(&params.log)?;

    let (tf_names, gen_names, instance) = parse_file(&params.file, params.iterations)?;

    eval_cov(tf_names, gen_names, instance, params.k, params.l, params.iterations, params.log);
    Ok(())
}

type Names = Vec<String>;

fn parse_file(path: &PathBuf, num: usize) -> Result<(Names, Names, BipartiteRegulatorProbing), Error> {
    let error = |msg| Err(Error::new(ErrorKind::Other, msg)); 

    let mut lines = BufReader::new(File::open(path)?).lines().filter_map(|x| -> Option<String> {
        if let Ok(line) = x {
            Some(line)
        } else {
            None
        }
    });

    

    let tf_names: Vec<String> = if let Some(header) = lines.next() {
        header.split('\t').skip(2).map(|s| s.to_string()).collect()
    } else {
        return error("Empty file!");
    };

    let na = tf_names.len();

    let mut gen_names: Vec<String> = Vec::new();

    let mut vs = 0u64;
    let binom_values: Vec<Vec<(u64, f64)>> = lines.map(|l| -> Vec<(u64, f64)> {
        let mut fields = l.split('\t');
        gen_names.push(fields.next().unwrap().to_string());
        let n = fields.next().unwrap().parse::<u64>().unwrap();
        if n > vs {
            vs = n;
        }
        fields.map(|f| -> (u64, f64) {
            let p = f.parse::<f64>().unwrap();
            (n, p)
        }).collect() 
    }).collect();

    let nb = gen_names.len();

    let binom_weights = |n: u64, p: f64| -> Vec<f64> {
        let binom = Binomial::new(p, n).unwrap();

        let mut total_weight = 0.0;
        let mut weights: Vec<f64> = (0..(vs - 1)).map(|i| {
            let p = binom.pmf(i);
            total_weight += p;
            p
        }).collect();

        weights.push(1.0 - total_weight);
        weights
    };

    let rng = &mut rand::thread_rng();

    let edges = (0..na).map(|a| -> Vec<WeightedDistribution> {
        (0..nb).map(|b| -> WeightedDistribution {
            let (n, p) = binom_values[b][a];
            WeightedDistribution::new(rng, &binom_weights(n, p), num)
        }).collect()
    }).collect();

    Ok((
        tf_names,
        gen_names,
        BipartiteRegulatorProbing::new(na, nb, vs as usize, edges)
    ))
}


fn eval_cov(tfs: Vec<String>, gens: Vec<String>, mut bpr: BipartiteRegulatorProbing, k: usize, l: usize, num: usize,log: PathBuf) {
    bpr.compute_namp_cov_policy(k, l);
    
    (0..num).for_each(|i| {
        let ins = bpr.create_instance(i);
        println!("Opt: {} in {}s", ins.get_opt_cov_value(l), ins.get_opt_cov_time());
        println!("Namp: {} in {}s", ins.eval_policy(bpr.get_policy(k, l).unwrap(), l), bpr.get_policy_time(k, l).unwrap());
        let (amp, time) = ins.adaptive_policy(k, l);
        println!("Amp: {} in {}s", amp, time);
    });
}
