use std::iter::Sum;
use std::ops::Mul;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Clone> Matrix<T> {
    /// returns the number of columns in the matrix
	pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
	}
    /// returns the number of rows in the matrix
	pub fn number_of_rows(&self) -> usize {
        self.0.len()
	}
    /// returns the `n`th row in the matrix.
	pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
	}
    /// returns the `n`th column in the matrix.
	pub fn col(&self, n: usize) -> Vec<T> {
        let mut result = Vec::new();
        for row in &self.0 {
            result.push(row[n].clone());
        }
        result
	}
}

impl<T: Clone + Mul<Output = T> + Sum> Mul for Matrix<T> {
    type Output = Option<Self>;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }
        let mut result = Vec::new();
        for i in 0..self.number_of_rows() {
            let mut new_row = Vec::new();
            for j in 0..rhs.number_of_cols() {
                let cell: T = self.row(i)
                    .into_iter()
                    .zip(rhs.col(j))
                    .map(|(a, b)| a * b)
                    .sum();
                new_row.push(cell);
            }
            result.push(new_row);
        }
        Some(Matrix(result))
    }
}