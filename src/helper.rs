pub type Probability = f64;
pub type Reward = f64;

/// Checks if two numbers of f64 are close enough to let their difference be considered a numerical error
pub fn is_close(a: f64, b: f64, tol: Option<f64>) -> bool {
    (b - a).abs() < tol.unwrap_or(1e-09)
}

/// Generates all possible combination of length n and values 0..v
pub fn combinations(n: usize, values: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![vec![]];
    }
    let mut val: Vec<Vec<usize>> = Vec::new();
    for v in 0..values {
        for vector in combinations(n - 1, values) {
            let mut x: Vec<usize> = Vec::with_capacity(n);
            x.push(v);
            for z in vector {
                x.push(z);
            }
            val.push(x);
        }
    }

    val
}

/// Get all possible combinations of true/false of length n (for subset calculations)
pub fn boolean_combination(n: usize) -> Vec<Vec<bool>> {
    if n == 0 {
        return vec![];
    }
    combinations(n, 2)
        .into_iter()
        .map(|v| v.into_iter().map(|u| u == 1).rev().collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>()
}

/// Converts a vector of boolen (binary reprensentation) to its equivalent number
pub fn boolean_array_to_usize(arr: &Vec<bool>) -> usize {
    let mut val: usize = 0;
    for k in 0..arr.len() {
        if arr[k] {
            val += 2_usize.pow(k as u32);
        }
    }

    val
}
