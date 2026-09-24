use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
	Nulla = 0,
	I = 1,
	V = 5,
	X = 10,
	L = 50,
	C = 100,
	D = 500,
	M = 1000,
}

const TABLE: [(u32, &[RomanDigit]); 13] = [
    (1000, &[M]),
    (900,  &[C, M]),
    (500,  &[D]),
    (400,  &[C, D]),
    (100,  &[C]),
    (90,   &[X, C]),
    (50,   &[L]),
    (40,   &[X, L]),
    (10,   &[X]),
    (9,    &[I, X]),
    (5,    &[V]),
    (4,    &[I, V]),
    (1,    &[I]),
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
	fn from(value: u32) -> Self {
        let mut roman_number:Vec<RomanDigit> = Vec::new();
        let mut n = value;
        if n == 0 {
            roman_number.push(RomanDigit::Nulla);
        } else {
            for (amount, symbols) in TABLE.into_iter() {
                while n >= amount {
                    roman_number.extend_from_slice(symbols);
                    n -= amount;
                }
            }
        }
		RomanNumber(roman_number)
    }
}
