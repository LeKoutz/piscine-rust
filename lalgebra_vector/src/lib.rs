use std::{iter::Sum, ops::{Add, Mul}};
use lalgebra_scalar::Scalar;

#[derive(Debug, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl <T: Scalar + Add<Output = T>> Add for Vector<T> {
    type Output = Option<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }
        let summed: Vec<T> = self.0.into_iter()
            .zip(rhs.0.into_iter())
            .map(|(a, b)| a + b)
            .collect();
        Some(Vector(summed))
    }
}

impl <T: Scalar + Mul<Output = T> + Sum> Vector<T> {
	pub fn dot(self, rhs: Self) -> Option<T> {
		if self.0.len() != rhs.0.len() {
            return None;
        }
        let result= self.0.into_iter()
            .zip(rhs.0.into_iter())
            .map(|(a, b)| a * b).sum();
        Some(result)
	}
}
