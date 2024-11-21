use std::{fs::File, io::{BufRead, BufReader, Error, ErrorKind}, path::PathBuf};

use bpr::{distributions::WeightedDistribution, model::BipartiteRegulatorProbing, GoalFunction};
use statrs::distribution::{Discrete, Poisson};
use structopt::StructOpt;



#[derive(Debug, StructOpt)]
struct Parameters {
    #[structopt(short = "f", long, parse(from_os_str))]
    file: PathBuf,

    #[structopt(short = "l", long, parse(from_os_str))]
    log: PathBuf,

    #[structopt(short = "i", long, default_value = "1")]
    iterations: usize,

    #[structopt(default_value = "COV")]
    goal: GoalFunction,

    #[structopt(short = "v", long, default_value = "10")]
    vs: u64,
}


fn main() -> std::io::Result<()> {
    let params = Parameters::from_args();

    let _ = std::fs::create_dir_all(&params.log)?;

    let (tf_names, gen_names, instance) = parse_file(&params.file, params.vs, params.iterations)?;

    Ok(())
}

type Names = Vec<String>;

fn parse_file(path: &PathBuf, vs: u64, num: usize) -> Result<(Names, Names, BipartiteRegulatorProbing), Error> {
    let error = |msg| Err(Error::new(ErrorKind::Other, msg)); 

    let mut lines = BufReader::new(File::open(path)?).lines().filter_map(|x| -> Option<String> {
        if let Ok(line) = x {
            Some(line)
        } else {
            None
        }
    });

    

    let gen_names: Vec<String> = if let Some(header) = lines.next() {
        header.split('\t').skip(1).map(|s| s.to_string()).collect()
    } else {
        return error("Empty file!");
    };

    let nb = gen_names.len();

    let poisson_weights = |lambda: f64| -> Vec<f64> {
        let poisson = Poisson::new(lambda).unwrap();

        let mut total_weight = 0.0;
        let mut weights: Vec<f64> = (0..(vs - 1)).map(|i| {
            let p = poisson.pmf(i);
            total_weight += p;
            p
        }).collect();

        weights.push(1.0 - total_weight);
        weights
    };

    let mut tf_names: Vec<String> = Vec::new();

    let rng = &mut rand::thread_rng();

    let edges: Vec<Vec<WeightedDistribution>> = lines.map(|l| -> Vec<WeightedDistribution> {
        let mut fields = l.split('\t');
        tf_names.push(fields.next().unwrap().to_string());
        fields.map(|f| -> WeightedDistribution {
            let lambda = f.parse::<f64>().unwrap();
            WeightedDistribution::new(rng, &poisson_weights(lambda), num)
        }).collect() 
    }).collect();

    let na = tf_names.len();

    Ok((
        tf_names,
        gen_names,
        BipartiteRegulatorProbing::new(na, nb, vs as usize, edges)
    ))
}
