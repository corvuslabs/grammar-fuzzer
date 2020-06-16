//! # Shared

use rand::seq::SliceRandom;
use std::collections::HashSet;

/// add_to_set returns a new set that includes the element
pub fn add_to_set<'a>(s: &HashSet<&'a str>, element: &'a str) -> HashSet<&'a str> {
    let mut new_set = s.clone();
    new_set.insert(element);
    new_set
}

/// selects randomly from the set of indicies of min value elements
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

/// selects randomly from the set of indicies of max value elements
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

/// random_element selects a random element satisfying the predicate
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_to_set() {
        let input = vec!["<string>", "<json>"].iter().cloned().collect();
        let result = add_to_set(&input, "<symbol>");
        let expected: HashSet<&str> = vec!["<string>", "<json>", "<symbol>"]
            .iter()
            .cloned()
            .collect();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_min_idx() {
        let result = min_idx(&vec![1.0, 2.0, 3.0, 4.0, 1.0]);
        let expected_idx = vec![0, 4];

        assert_eq!(expected_idx.contains(&result), true);
    }

    #[test]
    fn test_max_idx() {
        let result = max_idx(&vec![1.0, 2.0, 3.0, 4.0, 4.0]);
        let expected_idx = vec![3, 4];

        assert_eq!(expected_idx.contains(&result), true);
    }

    #[test]
    fn test_random_element() {
        let input = vec![1, 2, 3, 4, 5];
        let result = random_element(&input, |e| *e > 3);
        let expected_values = vec![Some(&4), Some(&5)];

        assert_eq!(expected_values.contains(&result), true);
    }
}
