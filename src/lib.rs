#![doc = include_str!("../README.md")]

use ndarray::{Array1, Array2};
use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

// Helper function to generate random data
fn parallel_randn(size: usize) -> Vec<f64> {
    (0..size)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            rng.sample(StandardNormal)
        })
        .collect()
}

/// Generates a random vector of the specified size:
pub fn randn_vector(size: usize) -> Array1<f64> {
    Array1::from_vec(parallel_randn(size))
}

/// Generates a random matrix of the specified rows and columns sizes:
pub fn randn_matrix(rows: usize, cols: usize) -> Array2<f64> {
    Array2::from_shape_vec((rows, cols), parallel_randn(rows * cols)).unwrap()
}

/// Generates a vector of random matrices of specified rows, columns, and number of simulations:
pub fn randn_matrices(rows: usize, cols: usize, sims: usize) -> Vec<Array2<f64>> {
    (0..sims)
        .into_par_iter()
        .map(|_| randn_matrix(rows, cols))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mean(data: &Array1<f64>) -> f64 {
        data.sum() / data.len() as f64
    }

    fn variance(data: &Array1<f64>, mean: f64) -> f64 {
        data.iter()
            .map(|&value| (value - mean).powi(2))
            .sum::<f64>()
            / data.len() as f64
    }

    fn standard_deviation(data: &Array1<f64>, mean: f64) -> f64 {
        variance(data, mean).sqrt()
    }

    #[test]
    fn test_vector_statistics() {
        let size: usize = 50_000;
        let vec = randn_vector(size);

        let mean_value = mean(&vec);
        let variance = variance(&vec, mean_value);
        let std_dev = variance.sqrt();
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
        let data = mat.as_slice().unwrap();

        let mean_value = mean(&Array1::from(data.to_vec()));
        let std_dev = standard_deviation(&Array1::from(data.to_vec()), mean_value);
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
