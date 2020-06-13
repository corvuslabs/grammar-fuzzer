//! # Shared

use rand::seq::SliceRandom;
use std::collections::HashSet;

pub fn add_to_set<'a>(s: &HashSet<&'a str>, e: &'a str) -> HashSet<&'a str> {
    let mut new_set = s.clone();
    new_set.insert(e);
    new_set
}

pub fn min_idx(vs: &Vec<f64>) -> usize {
    assert_eq!(vs.is_empty(), false);
    let min = vs
        .iter()
        .fold(f64::INFINITY, |min, c| if min < *c { min } else { *c });
    let min_idxs: Vec<usize> = vs
        .iter()
        .enumerate()
        .filter(|(_, v)| **v == min)
        .map(|(i, _)| i)
        .collect();
    *random_element(&min_idxs, |_| true).unwrap()
}

pub fn max_idx(vs: &Vec<f64>) -> usize {
    assert_eq!(vs.is_empty(), false);
    let max = vs
        .iter()
        .fold(f64::NEG_INFINITY, |max, c| if max > *c { max } else { *c });
    let max_idxs: Vec<usize> = vs
        .iter()
        .enumerate()
        .filter(|(_, v)| **v == max)
        .map(|(i, _)| i)
        .collect();
    *random_element(&max_idxs, |_| true).unwrap()
}

pub fn random_element<T, F>(vs: &Vec<T>, p: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
    let idxs: Vec<usize> = vs
        .iter()
        .enumerate()
        .filter(|(_, v)| p(v))
        .map(|(idx, _)| idx)
        .collect();
    // Found out about the SliceRandom trait (https://docs.rs/rand/0.7.3/rand/seq/trait.SliceRandom.html#tymethod.choose) from:
    // https://stackoverflow.com/questions/34215280/how-can-i-randomly-select-one-element-from-a-vector-or-array
    // https://stackoverflow.com/a/42272866
    let mut rng = rand::thread_rng();
    let idx = idxs.choose(&mut rng)?;
    Some(&vs[*idx])
}
