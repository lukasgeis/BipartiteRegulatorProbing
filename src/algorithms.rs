use crate::distributions::Distribution;

pub fn namp_probemax(data: &Vec<Distribution>) -> Vec<usize> {
    let mut ordering: Vec<(usize, f64)> = data
        .into_iter()
        .enumerate()
        .map(|(i, d)| (i, d.expected_value()))
        .collect();
    ordering.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    ordering.into_iter().map(|(i, _)| i).collect()
}
