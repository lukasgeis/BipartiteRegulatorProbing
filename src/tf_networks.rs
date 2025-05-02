use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
    path::PathBuf,
    time::Instant,
};

use bpr::{
    distributions::WeightedDistribution,
    model::{BipartiteRegulatorProbing, NUM_TOP_TUPLES},
};
use serde::Serialize;
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

    #[structopt(short = "l", default_value = "3")]
    l: usize,

    #[structopt(long)]
    noopt: bool,
}

fn main() -> std::io::Result<()> {
    let params = Parameters::from_args();

    let (tf_names, gen_names, instance) = parse_file(&params.file, params.iterations)?;

    match params.l {
        1 => eval_cov::<1>(
            tf_names,
            gen_names,
            instance,
            params.k,
            params.iterations,
            params.noopt,
        ),
        2 => eval_cov::<2>(
            tf_names,
            gen_names,
            instance,
            params.k,
            params.iterations,
            params.noopt,
        ),
        3 => eval_cov::<3>(
            tf_names,
            gen_names,
            instance,
            params.k,
            params.iterations,
            params.noopt,
        ),
        4 => eval_cov::<4>(
            tf_names,
            gen_names,
            instance,
            params.k,
            params.iterations,
            params.noopt,
        ),
        5 => eval_cov::<5>(
            tf_names,
            gen_names,
            instance,
            params.k,
            params.iterations,
            params.noopt,
        ),
        6 => eval_cov::<6>(
            tf_names,
            gen_names,
            instance,
            params.k,
            params.iterations,
            params.noopt,
        ),
        7 => eval_cov::<7>(
            tf_names,
            gen_names,
            instance,
            params.k,
            params.iterations,
            params.noopt,
        ),
        8 => eval_cov::<8>(
            tf_names,
            gen_names,
            instance,
            params.k,
            params.iterations,
            params.noopt,
        ),
        9 => eval_cov::<9>(
            tf_names,
            gen_names,
            instance,
            params.k,
            params.iterations,
            params.noopt,
        ),
        10 => eval_cov::<10>(
            tf_names,
            gen_names,
            instance,
            params.k,
            params.iterations,
            params.noopt,
        ),
        11 => eval_cov::<11>(
            tf_names,
            gen_names,
            instance,
            params.k,
            params.iterations,
            params.noopt,
        ),
        12 => eval_cov::<12>(
            tf_names,
            gen_names,
            instance,
            params.k,
            params.iterations,
            params.noopt,
        ),
        _ => panic!("l must be a value between 1 and 32"),
    };

    Ok(())
}

type Names = Vec<String>;

fn parse_file(
    path: &PathBuf,
    num: usize,
) -> Result<(Names, Names, BipartiteRegulatorProbing), Error> {
    let error = |msg| Err(Error::new(ErrorKind::Other, msg));

    let mut lines = BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|x| -> Option<String> {
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
    let binom_values: Vec<Vec<(u64, f64)>> = lines
        .map(|l| -> Vec<(u64, f64)> {
            let mut fields = l.split('\t');
            gen_names.push(fields.next().unwrap().to_string());
            let n = fields.next().unwrap().parse::<u64>().unwrap();
            if n > vs {
                vs = n;
            }
            fields
                .map(|f| -> (u64, f64) {
                    let p = f.parse::<f64>().unwrap();
                    (n, p)
                })
                .collect()
        })
        .collect();

    let nb = gen_names.len();

    let binom_weights = |n: u64, p: f64| -> Vec<f64> {
        let binom = Binomial::new(p, n).unwrap();

        let mut total_weight = 0.0;
        let mut weights: Vec<f64> = (0..(vs - 1))
            .map(|i| {
                let p = binom.pmf(i);
                total_weight += p;
                p
            })
            .collect();

        weights.push(1.0 - total_weight);
        weights
    };

    let rng = &mut rand::rng();

    let edges = (0..na)
        .map(|a| -> Vec<WeightedDistribution> {
            (0..nb)
                .map(|b| -> WeightedDistribution {
                    let (n, p) = binom_values[b][a];
                    WeightedDistribution::new(rng, &binom_weights(n, p), num)
                })
                .collect()
        })
        .collect();

    Ok((
        tf_names,
        gen_names,
        BipartiteRegulatorProbing::new(na, nb, vs as usize, edges),
    ))
}

fn eval_cov<const NUM: usize>(
    tfs: Vec<String>,
    _gens: Vec<String>,
    mut bpr: BipartiteRegulatorProbing,
    k: usize,
    num: usize,
    exclude_opt: bool,
) where
    [String; NUM]: Serialize,
{
    bpr.compute_namp_cov_policy(k, NUM);

    (0..num).for_each(|i| {
        let ins = bpr.create_instance(i);

        let res = if exclude_opt {
            let namp = ins.find_top_tuples::<NUM>(bpr.get_policy(k, NUM).unwrap());

            let timer = Instant::now();
            let amp = ins.adaptive_policy_regulators(k, NUM);
            let time = AlgoTimes(
                timer.elapsed().as_secs_f64(),
                bpr.get_policy_time(k, NUM).unwrap(),
            );

            let amp = ins.find_top_tuples::<NUM>(&amp);

            TfNetworkResult {
                k,
                opt: Default::default(),
                namp: (namp, tfs.as_ref()).into(),
                amp: (amp, tfs.as_ref()).into(),
                time,
            }
        } else {
            let opt = ins.top_opt_tuples::<NUM>();
            let namp = ins.find_top_tuples::<NUM>(bpr.get_policy(k, NUM).unwrap());

            let timer = Instant::now();
            let amp = ins.adaptive_policy_regulators(k, NUM);
            let time = AlgoTimes(
                timer.elapsed().as_secs_f64(),
                bpr.get_policy_time(k, NUM).unwrap(),
            );

            let amp = ins.find_top_tuples::<NUM>(&amp);

            TfNetworkResult {
                k,
                opt: (opt, tfs.as_ref()).into(),
                namp: (namp, tfs.as_ref()).into(),
                amp: (amp, tfs.as_ref()).into(),
                time,
            }
        };

        println!("{}", serde_json::to_string(&res).unwrap());
    });
}

#[derive(Debug, Serialize)]
struct ConstantTFResult<const NUM: usize>([String; NUM], usize)
where
    [String; NUM]: Serialize;

#[derive(Debug, Serialize)]
struct TopTuples<const NUM: usize>([ConstantTFResult<NUM>; NUM_TOP_TUPLES])
where
    [String; NUM]: Serialize;

impl<const NUM: usize> From<([([usize; NUM], usize); NUM_TOP_TUPLES], &[String])> for TopTuples<NUM>
where
    [String; NUM]: Serialize,
{
    fn from(value: ([([usize; NUM], usize); NUM_TOP_TUPLES], &[String])) -> Self {
        TopTuples(value.0.map(|(arr, val)| -> ConstantTFResult<NUM> {
            ConstantTFResult(arr.map(|x| value.1[x].clone()), val)
        }))
    }
}

impl<const NUM: usize> Default for TopTuples<NUM>
where
    [String; NUM]: Serialize,
{
    fn default() -> Self {
        let names = [String::new()];
        ([([0usize; NUM], 0usize); NUM_TOP_TUPLES], names.as_ref()).into()
    }
}

#[derive(Debug, Serialize)]
struct AlgoTimes(f64, f64);

#[derive(Serialize, Debug)]
struct TfNetworkResult<const NUM: usize>
where
    [String; NUM]: Serialize,
{
    k: usize,
    opt: TopTuples<NUM>,
    amp: TopTuples<NUM>,
    namp: TopTuples<NUM>,
    time: AlgoTimes,
}
