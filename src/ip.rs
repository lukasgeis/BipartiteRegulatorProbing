use highs::{HighsModelStatus, RowProblem};
use itertools::Itertools;

pub fn solve_cov_instance(na: usize, nb: usize, l: usize, weights: &[Vec<usize>]) -> usize {
    debug_assert_eq!(weights.len(), na);
    for a in 0..na {
        debug_assert_eq!(weights[a].len(), nb);
    }

    let mut model = RowProblem::default();

    // Variable for each regulator
    // Normally integral - but here relaxed
    let regs = (0..na)
        .map(|_| model.add_column(0.0, 0.0..=1.0))
        .collect_vec();

    // Variable for each edge
    let edges = (0..na)
        .map(|a| {
            (0..nb)
                .map(|b| model.add_column(weights[a][b] as f64, 0.0..=1.0))
                .collect_vec()
        })
        .collect_vec();

    for b in 0..nb {
        // Each Gen/Position can be covered at most once
        model.add_row(0.0..=1.0, (0..na).map(|a| (edges[a][b], 1.0)));

        for a in 0..na {
            // edges[a][b] <= regs[a]
            model.add_row(..=0.0, [(edges[a][b], 1.0), (regs[a], -1.0)]);
        }
    }

    model.add_row(0.0..=(l as f64), (0..na).map(|a| (regs[a], 1.0)));

    let solved = model.optimise(highs::Sense::Maximise).solve();

    assert_eq!(solved.status(), HighsModelStatus::Optimal);

    let solution = solved.get_solution();

    let sol_vars = solution.columns();

    //let mut sol_regs = Vec::with_capacity(l);
    let mut obj_val = 0.0;
    for a in 0..na {
        // Values should be integral, but sometimes the value is not 1.0 due to floating point approx
        //if sol_vars[a] > 0.5 {
        //    sol_regs.push(a);
        //}

        for b in 0..nb {
            let idx = na + nb * a + b;
            obj_val += (weights[a][b] as f64) * sol_vars[idx];
        }
    }

    //let real_obj = (0..nb).map(|b| sol_regs.iter().map(|&a| weights[a][b]).max().unwrap()).sum();
    // assert_eq!(real_obj, obj_val as usize);
    //(real_obj, sol_regs)

    obj_val.floor() as usize
}
