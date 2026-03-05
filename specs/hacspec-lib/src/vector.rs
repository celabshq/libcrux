use crate::{field::FieldElement, ring::PolynomialRingElement};
use std::ops::{self, Index, IndexMut};

#[derive(Debug, Clone, Copy)]
pub struct Vector<F: FieldElement, const COEFFICIENTS: usize, const SIZE: usize> {
    values: [PolynomialRingElement<F, COEFFICIENTS>; SIZE],
}
impl<const LEN: usize, F: FieldElement, const COEFFICIENTS: usize> Vector<F, COEFFICIENTS, LEN> {
    pub const ZERO: Self = Self {
        values: [PolynomialRingElement::<F, COEFFICIENTS>::ZERO; LEN],
    };

    pub fn len(&self) -> usize {
        LEN
    }

    pub fn iter(&self) -> std::slice::Iter<'_, PolynomialRingElement<F, COEFFICIENTS>> {
        self.values.iter()
    }

    pub fn into_iter(&self) -> std::array::IntoIter<PolynomialRingElement<F, COEFFICIENTS>, LEN> {
        self.values.into_iter()
    }
}

impl<const LEN: usize, F: FieldElement, const COEFFICIENTS: usize> Index<usize>
    for Vector<F, COEFFICIENTS, LEN>
{
    type Output = PolynomialRingElement<F, COEFFICIENTS>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<const LEN: usize, F: FieldElement, const COEFFICIENTS: usize> IndexMut<usize>
    for Vector<F, COEFFICIENTS, LEN>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl<const LEN: usize, F: FieldElement, const COEFFICIENTS: usize> ops::Add
    for Vector<F, COEFFICIENTS, LEN>
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..LEN {
            self.values[i] = self.values[i] + rhs.values[i];
        }
        self
    }
}
