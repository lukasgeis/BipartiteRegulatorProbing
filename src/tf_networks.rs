use std::{fs::File, io::{BufRead, BufReader, Error, ErrorKind}, path::PathBuf, time::Instant};

use bpr::{distributions::WeightedDistribution, model::{BipartiteRegulatorProbing, NUM_TOP_TRIPLES}};
use serde_derive::Serialize;
use statrs::distribution::{Binomial, Discrete};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Parameters {
    #[structopt(long, parse(from_os_str))]
    file: PathBuf,

    #[structopt(short = "i", long, default_value = "1")]
    iterations: usize,

    #[structopt(short = "k", default_value = "10")]
    k: usize,
}


fn main() -> std::io::Result<()> {
    let params = Parameters::from_args();

    let (tf_names, gen_names, instance) = parse_file(&params.file, params.iterations)?;

    eval_cov(tf_names, gen_names, instance, params.k, params.iterations);
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


fn eval_cov(tfs: Vec<String>, _gens: Vec<String>, mut bpr: BipartiteRegulatorProbing, k: usize, num: usize) {
    let l = 3usize;

    bpr.compute_namp_cov_policy(k, l);
    
    (0..num).for_each(|i| {
        let ins = bpr.create_instance(i);

        let opt = ins.top_opt_triples();
        let namp = ins.find_top_triples(bpr.get_policy(k, 3).unwrap());

        let timer = Instant::now();
        let amp = ins.adaptive_policy_regulators(k, 3);
        let time = AlgoTimes(timer.elapsed().as_secs_f64(), bpr.get_policy_time(k, 3).unwrap());

        let amp = ins.find_top_triples(&amp);


        let res = TfNetworkResult {
            k,
            opt: (opt, &tfs).into(),
            namp: (namp, &tfs).into(),
            amp: (amp, &tfs).into(),
            time
        };

        println!("{}", serde_json::to_string(&res).unwrap());
    });
}

#[derive(Debug, Serialize)]
struct TripleTFResult((String, String, String), usize);

#[derive(Debug, Serialize)]
struct TopTriples([TripleTFResult; NUM_TOP_TRIPLES]);

impl From<([(usize, usize, usize, usize); NUM_TOP_TRIPLES], &Vec<String>)> for TopTriples {
    fn from(value: ([(usize, usize, usize, usize); NUM_TOP_TRIPLES], &Vec<String>)) -> Self {
        TopTriples(value.0.map(|x| -> TripleTFResult {
            TripleTFResult(
                (
                    value.1[x.0].clone(),
                    value.1[x.1].clone(),
                    value.1[x.2].clone()
                ),
                x.3
            )
        }))
    }
}

#[derive(Debug, Serialize)]
struct AlgoTimes(f64, f64);

#[derive(Serialize, Debug)]
struct TfNetworkResult {
    k: usize,
    opt: TopTriples,
    amp: TopTriples,
    namp: TopTriples,
    time: AlgoTimes,
}
