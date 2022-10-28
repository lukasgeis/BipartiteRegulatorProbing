use crate::distributions::Distribution;

pub fn namp_for_probemax(distributions: &Vec<Distribution>, k: usize) -> Vec<usize> {
    let mut expected_values: Vec<(usize, f64)> = distributions
        .into_iter()
        .enumerate()
        .map(|(i, d)| -> (usize, f64) { (i, d.expected_value()) })
        .collect();

    expected_values.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    expected_values.truncate(k);

    expected_values.into_iter().map(|(a, _)| a).collect()
}
