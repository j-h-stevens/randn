```markdown
# Randn Rust Library

The Randn Rust library provides utilities for generating random vectors, matrices, and a list of matrices with values sampled from the standard normal distribution. It leverages the power of parallel processing to efficiently generate large sets of data.

## Dependencies

This library is built upon a set of Rust crates:
- `rand` and `rand_distr` for random number generation.
- `nalgebra` for array operations and representations. (Switched from `ndarray`)
- `rayon` for parallel computing capabilities.

## Features

1. **Parallel Random Data Generation**
   
   The library uses parallel processing to generate random numbers faster:
   ```rust
   fn parallel_randn(size: usize) -> Vec<f64> {...}
   ```

2. **Random Vector Generation**

   Generates a random vector of the specified size using ndarray's `Array1`:
   ```rust
   pub fn randn_vector(size: usize) -> Array1<f64> {...}
   ```

3. **Random Matrix Generation**

   Generates a random matrix of the specified rows and columns sizes using ndarray's `Array2`:
   ```rust
   pub fn randn_matrix(rows: usize, cols: usize) -> Array2<f64> {...}
   ```

4. **Generation of Multiple Random Matrices**

   Generates a vector of random matrices of specified rows, columns, and number of simulations using ndarray's `Array2`:
   ```rust
   pub fn randn_matrices(rows: usize, cols: usize, sims: usize) -> Vec<Array2<f64>> {...}
   ```

## Usage

To use the RandN library in your project, include it in your Cargo.toml dependencies and then import the necessary functions in your Rust code.

## Contribution

Contributions to the RandN library are welcomed! Please open an issue or submit a pull request on the GitHub repository.

## License

This library is licensed under the MIT License.
```