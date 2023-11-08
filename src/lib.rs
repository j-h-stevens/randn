#![doc = include_str!("../README.md")]

extern crate nalgebra as na;
extern crate rand;
extern crate rand_distr;
extern crate rayon;

use nalgebra::{DMatrix, DVector};
use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;
use rayon::prelude::*;

// Helper function to generate random data in parallel
fn parallel_randn(size: usize) -> Vec<f64> {
    (0..size)
        .into_par_iter()
        .map_init(thread_rng, |rng, _| rng.sample(StandardNormal))
        .collect()
}

/// Generates a random vector of the specified size in parallel:
pub fn randn_vector(size: usize) -> DVector<f64> {
    DVector::from_vec(parallel_randn(size))
}

/// Generates a random matrix of the specified rows and columns sizes in parallel:
pub fn randn_matrix(rows: usize, cols: usize) -> DMatrix<f64> {
    let size = rows * cols;
    DMatrix::from_vec(cols, rows, parallel_randn(size))
}

/// Generates a vector of random matrices of specified rows, columns, and number of simulations:
pub fn randn_matrices(rows: usize, cols: usize, sims: usize) -> Vec<DMatrix<f64>> {
    (0..sims)
        .into_par_iter()
        .map(|_| randn_matrix(rows, cols))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use na::DVector;

    // Statistical functions adapted for `nalgebra`'s `DVector`
    fn mean(data: &DVector<f64>) -> f64 {
        data.iter().sum::<f64>() / data.len() as f64
    }

    fn variance(data: &DVector<f64>, mean: f64) -> f64 {
        data.iter()
            .map(|&value| (value - mean).powi(2))
            .sum::<f64>()
            / data.len() as f64
    }

    fn standard_deviation(data: &DVector<f64>, mean: f64) -> f64 {
        variance(data, mean).sqrt()
    }

    #[test]
    fn test_vector_statistics() {
        let size: usize = 50_000;
        let vec = randn_vector(size);

        let mean_value = mean(&vec);
        let std_dev = standard_deviation(&vec, mean_value);
        let epsilon = 0.01;

        // Assert that the mean and standard deviation are approximately 0 and 1 respectively.
        assert!(
            (mean_value - 0.0).abs() < epsilon,
            "Mean is not approximately 0: actual mean = {}",
            mean_value
        );
        assert!(
            (std_dev - 1.0).abs() < epsilon,
            "Standard deviation is not approximately 1: actual std_dev = {}",
            std_dev
        );
    }

    #[test]
    fn test_matrix_statistics() {
        let (rows, cols): (usize, usize) = (50, 1000);
        let mat = randn_matrix(rows, cols);
        let data = DVector::from_iterator(rows * cols, mat.iter().cloned());

        let mean_value = mean(&data);
        let std_dev = standard_deviation(&data, mean_value);
        let epsilon = 0.01;

        // Assert that the mean and standard deviation are approximately 0 and 1 respectively.
        assert!(
            (mean_value - 0.0).abs() < epsilon,
            "Mean is not approximately 0: actual mean = {}",
            mean_value
        );
        assert!(
            (std_dev - 1.0).abs() < epsilon,
            "Standard deviation is not approximately 1: actual std_dev = {}",
            std_dev
        );
    }
}
