use std::{fmt, str::FromStr};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

impl FromStr for BloodType {
	type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antigen_str, rh_str) = s.split_at_checked(s.len()-1).ok_or(())?;
        let antigen = match antigen_str {
            "A" => Antigen::A,
            "B" => Antigen::B,
            "AB" => Antigen::AB,
            "O" => Antigen::O,
            _ => return Err(())
        };
        let rh_factor = match rh_str {
            "+" => RhFactor::Positive,
            "-" => RhFactor::Negative,
            _ => return Err(())
        };
        Ok(Self {
            antigen,
            rh_factor
        })
    }
}

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };
        let sign = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{antigen}{sign}")
    }
}

impl BloodType {
	pub fn can_receive_from(self, other: Self) -> bool {
        let antigen_ok = match self.antigen {
            Antigen::AB => true,
            Antigen::A => matches!(other.antigen, Antigen::A | Antigen::O),
            Antigen::B => matches!(other.antigen, Antigen::B | Antigen::O),
            Antigen::O => other.antigen == Antigen::O
        };
        let rh_factor_ok = match self.rh_factor {
            RhFactor::Negative => other.rh_factor == RhFactor::Negative,
            RhFactor::Positive => true
        };
		antigen_ok && rh_factor_ok
	}

    pub fn donors(self) -> Vec<Self> {
        let antigens = [Antigen::A, Antigen::B, Antigen::AB, Antigen::O];
        let rh_factors = [RhFactor::Positive, RhFactor::Negative];
        let mut blood_types = Vec::new();
        for &antigen in &antigens {
            for &rh_factor in &rh_factors {
                blood_types.push(BloodType {antigen, rh_factor});
            }
        }
        blood_types.into_iter().filter(|&b| self.can_receive_from(b)).collect()
    }

    fn can_donate_to(self, other: Self) -> bool {
        other.can_receive_from(self)
    }

	pub fn recipients(self) -> Vec<Self> {
		let antigens = [Antigen::A, Antigen::B, Antigen::AB, Antigen::O];
        let rh_factors = [RhFactor::Positive, RhFactor::Negative];
        let mut blood_types = Vec::new();
        for &antigen in &antigens {
            for &rh_factor in &rh_factors {
                blood_types.push(BloodType {antigen, rh_factor});
            }
        }
        blood_types.into_iter().filter(|&b| self.can_donate_to(b)).collect()
	}
}
