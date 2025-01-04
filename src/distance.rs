use pyo3::prelude::*;

#[pyfunction]
pub fn levenshtein(x: String, y: String) -> usize {
    let mut cost = vec![vec![0; y.len() + 1]; x.len() + 1];
    for i in 0..=x.len() {
        cost[i][0] = i;
    }
    for j in 0..=y.len() {
        cost[0][j] = j;
    }
    for (i, c1) in x.chars().enumerate() {
        for (j, c2) in y.chars().enumerate() {
            cost[i + 1][j + 1] = (cost[i][j] + if c1 == c2 { 0 } else { 1 })
                .min(cost[i][j + 1] + 1)
                .min(cost[i + 1][j] + 1);
        }
    }
    cost[x.len()][y.len()]
}
