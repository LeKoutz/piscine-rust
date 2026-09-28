use std::ops::{Add, Sub, Mul};
use lalgebra_scalar::Scalar;
use matrix::Matrix;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Wrapper<const W: usize, const H: usize, T: Scalar<Item = T>>(pub Matrix<W, H, T>);

impl<const W: usize, const H: usize, T: Scalar<Item = T>> From<[[T; W]; H]> for Wrapper<W, H, T> {
    fn from(value: [[T; W]; H]) -> Self {
        Self(Matrix(value))
    }
}

impl<const W: usize, const H: usize, T: Copy + Add<Output = T> + Scalar<Item = T>> Add for Wrapper<W, H, T> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        for row in 0..H {
            for col in 0..W {
                self.0.0[row][col] = self.0.0[row][col] + rhs.0.0[row][col];
            }
        }
        self
    }
}

impl<const W: usize, const H: usize, T: Copy + Sub<Output = T> + Scalar<Item = T>> Sub for Wrapper<W, H, T> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        for row in 0..H {
            for col in 0..W {
                self.0.0[row][col] = self.0.0[row][col] - rhs.0.0[row][col];
            }
        }
        self
    }
}

impl<const S: usize, T: Copy + Add<Output = T> + Mul<Output = T> + Scalar<Item = T>> Mul for Wrapper<S, S, T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Matrix::<S, S, T>::zero();
        for row in 0..S {
            for col in 0..S {
                for k in 0..S {
                    result.0[row][col] = result.0[row][col] + self.0.0[row][k] * rhs.0.0[k][col];
                }
            }
        }
        Wrapper(result)
    }
}
