use std::fmt;

#[derive(Debug)]
pub struct Player<'a> {
	pub name: &'a str,
	pub strength: f64,
	pub score: u32,
	pub money: u32,
	pub weapons: Vec<&'a str>,
}

pub struct Fruit {
	pub weight_in_kg: f64,
}

pub struct Meat {
	pub weight_in_kg: f64,
	pub fat_content: f64,
}

impl Player<'_> {
	pub fn eat(&mut self, food: impl Food) {
		self.strength += food.gives();
	}
}

impl fmt::Display for Player<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "Strength: {}, Score: {}, Money: {}", self.strength, self.score, self.money)?;
        write!(f, "Weapons: {:?}", self.weapons)?;
        Ok(())
    }
}

pub trait Food {
	fn gives(&self) -> f64;
}

impl Food for Fruit {
	fn gives(&self) -> f64 {
		self.weight_in_kg * 4.0
	}
}

impl Food for Meat {
	fn gives(&self) -> f64 {
        let fat = self.weight_in_kg * self.fat_content;
        let protein = (1.0 - self.fat_content) * self.weight_in_kg;
		(fat * 9.0) + (protein * 4.0)
	}
}
