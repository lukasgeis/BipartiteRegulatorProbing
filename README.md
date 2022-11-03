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

### Running the algorithms
As there are multiple possibilities of choosing $k$ and $\ell$ for every graph with $n_A \mathit{Regulators}$, there are multiple ways to run algorithms all using the same main binary created.
The standard prefix is always
```bash
target/release/bpr --input <Input Graph file> \
    --log <Path to log-file> \
    --iterations <Number of Instances> \
    --algorithm <ALGORITHM> 
    [--input-time <Path to log-file>]
    [--exclude-opt]
```
where input-time is a logfile to log the time it takes to read the graph. For a list of which algorithms to use, see [Algorithms](#algorithms). To specify which $k$ and $\ell$ to run on, you can either use 
```bash
-k <Number> -l <Number>
```
to run on a specific setting, or use 
```bash
--bruteforce
```
to run every possible combination of $k$ and $\ell$ on this instance. And lastly, you can use
```bash
--fraction <Number>
```
to run on every possible fraction-combination of $k$ and $\ell$. Namely, if `fraction` $= 2$, then the algorithms will run on $k = n_A, \frac{1}{2}n_A$ and $\ell = k, \frac{1}{2}k$.



<a name="algorithms" ></a>
### Algorithms

Input | Name | Adaptive? | Runtime | Value | Source
--- | --- | --- | --- | --- | ---
AMP | AdaptiveMyopicPolicy | Yes | $\mathcal{O}(n_A \cdot (k + \log n))$ | $\frac{e - 1}{e}OPT_A$ | [SMSM](https://arxiv.org/abs/0908.2788)
NAMP | NonAdaptiveMyopicPolicy | No | $\mathcal{O}(n_A\log n_A)$ | $\frac{e - 1}{2e}OPT_A$ | [SMSM](https://arxiv.org/abs/0908.2788)
SCG | StochasticContinousGreedy | No | $\mathcal{O}(k^3n_A^5\log n_A\log n_A)$ | $(\frac{e - 1}{e} - \frac{1}{n_A})OPT_A$ | [SMSM](https://arxiv.org/abs/0908.2788)
MDP | MarkovDevisionProcess | Yes | $\Omega((2 \cdot \|\mathcal{V}\|)^{n_A})$ | $OPT_A$ | [MDP](https://en.wikipedia.org/wiki/Markov_decision_process)

Additionally, you can use OPT to get the optimal value of this instance. Furthermore to run `AMP` and `NAMP`, you can use `FAST`, to run `AMP`, `NAMP`, and `SCG`, you can use `POLY` and to run all algorithms, you can use `ALL`. Note that `OPT` is always included except when `--exclude-opt` is specified

### Goal Functions
At the moment, there are $3$ possible goal functions. $f_{max}, f_{sum}$ which both reduce to [Top-$\ell$-ProbeMax](https://arxiv.org/pdf/2007.13121.pdf) and $f_{cov}$ which reduces to [MaximumCoverage](https://en.wikipedia.org/wiki/Maximum_coverage_problem) but is not implemented yet.

For $f_{max}$ and $f_{sum}$, each $\mathit{Regulator }\,\, a \in A$ is assigned an independent value, namely the maximum or the sum of all its incident edges. After that, we have to choose $\ell \,\,\mathit{Regulators}$ to maximize the sum of their values. 

### Jobs
The jobs folder contains all bash files to run the algorithms for comparison on the Goethe-HHLR cluster.