# BipartiteRegulatorProbing

We get a complete bipartite graph $G = (A \cup B, A \times B)$ with a set $A$ of $\mathit{Regulators}$ and a set $B$ of $\mathit{Positions}$. Every edge $(a,b) \in A \times B$ has an independent discrete distribution $D_{a,b}$ over the same support $\mathcal{V} :=$ { $0,...,|\mathcal{V}| - 1$ }. We know $D_{a,b}$ but not their edge weight realizations $w_{a,b} \sim D_{a,b}$. We can now $\mathit{probe}$ $k$ $\mathit{Regulators}$ thus revealing their incident edge weights. At the end, we have to choose $\ell$ $\mathit{Regulators}$ among the $\mathit{probed}$ ones to maximize a given set goal function $f$.

## Usage

The algorithms are implemented in Rust. Therefore, Rust must be installed in advance. On most Unix-like systems, you can install Rust with:
```bash
curl --proto '=https' --t1sv1.2 -sSf https://sh.rustup.rs | sh
```
After that you can modify the code as you wish and compile using the preinstalled `cargo` package manager:
```bash
cargo build --release
```

### Input Format

The input format is strictly dictated and cannot be changed without changing the code. You can create Instances of `BPR` using Python3 and [scripts/createData.py](scripts/createData.py) with the following command:
```bash
python3 scripts/createData.py --number <Number of Graphs> \
    --na_min <Minimum Number of Regulators> \
    --na_max <Maximum Number of Regulators> \
    --nb_min <Minimum Number of Positions> \
    --nb_max <Maximum Number of Positions> \
    --vs_min <Minimum Size of Support> \
    --vs_max <Maximum Size of Support> \
    --output <Output Directory> 
    [--name <Custom Name>]
```
You can see an example file here: [EXAMPLE](EXAMPLE)

Alternatively, you can create graph instances at runtime with prespecified parameters

### Running the algorithms
As there are multiple possibilities of choosing $k$ and $\ell$ for every graph with $n_A \mathit{Regulators}$, there are multiple ways to run algorithms all using the same main binary created.
The standard prefix is always
```bash
target/release/bpr --file <Input Graph file> \
    --log <Path to log-file> \
    --instances <Number of Instances> \
    --goal <Goal Function> \
    --algorithm <Algorithm> \
    --parameters <Number> \
    [--not-opt]
```
`Parameters` is used to run on every possible fraction-combination of $k$ and $\ell$. Namely, if `parameters` $= 2$, then the algorithms will run on $k = \frac{1}{3}n_A, \frac{2}{3}n_A$ and $\ell = \frac{1}{3}k, \frac{2}{3}k, k$.

If you wish to create graph instances at runtime, instead use
```bash
target/release/bpr --log <Path to log-file> \
    --na <Number of Regulators> \
    --nb <Number of Positions> \
    --vs <Size of Support> \
    --iterations <Number of Graph Instances> \
    --instances <Number of Instances per Graph Instance> \
    --parameters <Parameters as above> \
    --goal <Goal Function> \
    --algorithm <Algorithm> 
    [--poisson] <Should Support model a Poisson Distribution>
    [--not-opt]
```


<a name="algorithms" ></a>
### Algorithms


Goal | Input | Name | Runtime | Approximation Factor | Source
--- | --- | --- | --- | --- | ---
MAX / SUM | OPT | OptimalOfflineAlgorithm | $\mathcal{O}(n_A \cdot \log n_A)$ | $OPT$ | -
MAX / SUM | AMP | AdaptiveMyopicPolicy | $\mathcal{O}(n_A \cdot (k + \log n_A))$ | $\frac{e - 1}{e}OPT_A$ | [SMSM](https://arxiv.org/abs/0908.2788)
MAX / SUM | NAMP | NonAdaptiveMyopicPolicy | $\mathcal{O}(n_A \cdot \log n_A)$ | $\frac{e - 1}{2e}OPT_A$ | [SMSM](https://arxiv.org/abs/0908.2788)
COV | OPT | OptimalOfflineAlgorithm | $\mathcal{O}(\ell \cdot n_A \cdot n_B)$ | $\frac{e - 1}{e}OPT$ | [MSM](https://www.cs.toronto.edu/~eidan/papers/submod-max.pdf)
COV | AMP | AdaptiveMyopicPolicy | $\mathcal{O}(k^2 \cdot \ell \cdot n_A \cdot n_B)$ | - | [SMSM](https://arxiv.org/abs/0908.2788)
COV | NAMP | NonAdaptiveMyopicPolicy | $\mathcal{O}(k^2 \cdot \ell \cdot n_A \cdot n_B)$ | - | [SMSM](https://arxiv.org/abs/0908.2788)

To run all algorithms on a specified goal, use `ALL` and use `--not-opt` if you do not want to run `OPT` - otherwise it is always run and logged. 

### Goal Functions
There are $3$ possible goal functions. $f_{max}, f_{sum}$ which both reduce to [Top-l-ProbeMax](https://arxiv.org/pdf/2007.13121.pdf) and $f_{cov}$ which reduces to a variation of [MaximumCoverage](https://en.wikipedia.org/wiki/Maximum_coverage_problem).

For $f_{max}$ and $f_{sum}$, each $\mathit{Regulator }$  $a \in A$ is assigned an independent value, namely the maximum or the sum of all its incident edges. After that, we have to choose $\ell$ $\mathit{Regulators}$ to maximize the sum of their values. 

For $f_{cov}$, each $\mathit{Position}$ $b \in B$ is assigned the value of the highest incident edge to a $\mathit{Regulator}$ $a \in S$ in the chosen probed subset $S \subseteq A$. We have to choose $\ell$ probed $\mathit{Regulators}$ to maximize the sum of all $\mathit{Position}$-values.

### Jobs
The `jobs` folder contains all bash files to run the algorithms for comparison on the [Goethe-HHLR](https://csc.uni-frankfurt.de/wiki/doku.php?id=public:start) cluster.

### Scripts
The scripts folder contains all script files. Note that [Python](https://www.python.org/) must be installed beforehand. The scripts are:
* Data Generation using `createData.py`
* Data Compression using `bprTimeCompression.py`, `bprParametersCompression.py` or `bprValuesCompression` to compress logged results into small data that then can be plotted
* Plotting Data using `timePlot.py`, `parametersPlot.py` or `valuesPlot.py`

Installing the necessary python packages can be done via
```bash
pip install -r scripts/requirements
```