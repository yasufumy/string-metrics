use pyo3::prelude::*;

#[pyfunction]
pub fn levenshtein(s1: String, s2: String) -> usize {
    let mut cost = vec![vec![0; s2.len() + 1]; s1.len() + 1];
    for i in 0..=s1.len() {
        cost[i][0] = i;
    }
    for j in 0..=s2.len() {
        cost[0][j] = j;
    }
    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            cost[i + 1][j + 1] = (cost[i][j] + if c1 == c2 { 0 } else { 1 })
                .min(cost[i][j + 1] + 1)
                .min(cost[i + 1][j] + 1);
        }
    }
    cost[s1.len()][s2.len()]
}
