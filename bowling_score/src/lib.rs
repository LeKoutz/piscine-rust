#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u32>,
    frame: u32,
    pins_left: u32,
    rolls_in_frame: u32,
    fill_balls: u32,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            rolls: Default::default(),
            frame: 1,
            pins_left: 10,
            rolls_in_frame: 2,
            fill_balls: 0,
        }
    }

    pub fn roll(&mut self, pins: u32) -> Result<(), Error> {
        if pins > self.pins_left {
            return Err(Error::NotEnoughPinsLeft);
        } else if self.frame == 10 && self.rolls_in_frame == 0 && self.fill_balls == 0 {
            return Err(Error::GameComplete);
        }
        self.pins_left -= pins;
        self.rolls.push(pins);
        if self.frame < 10 {
            self.rolls_in_frame -= 1;
            if self.rolls_in_frame == 0 || self.pins_left == 0 {
                self.frame += 1;
                self.rolls_in_frame = 2;
                self.pins_left = 10;
            }
        } else if self.frame == 10 && self.fill_balls > 0 {
            self.fill_balls -= 1;
            if self.rolls_in_frame > 0 {
                self.rolls_in_frame -= 1;
            }
            if self.pins_left == 0 {
                self.pins_left = 10;
            }
        } else {
            if pins == 10 {
                self.fill_balls = 2;
                self.pins_left = 10;
            } else if self.pins_left == 0 && self.rolls_in_frame == 1 {
                self.fill_balls = 1;
                self.pins_left = 10;
            }
            if self.rolls_in_frame > 0 {
                self.rolls_in_frame -= 1;
            }
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u32> {
        // game must actually be finished
        if !(self.frame == 10 && self.rolls_in_frame == 0 && self.fill_balls == 0) {
            return None;
        }

        let mut total = 0;
        let mut i = 0; // index into self.rolls, marks the start of the current frame

        for _frame in 0..10 {
            if self.rolls[i] == 10 {
                // strike: 10 + next two rolls, advance by 1 (this frame only used 1 roll)
                total += 10 + self.rolls[i + 1] + self.rolls[i + 2];
                i += 1;
            } else if self.rolls[i] + self.rolls[i + 1] == 10 {
                // spare: 10 + next roll, advance by 2
                total += 10 + self.rolls[i + 2];
                i += 2;
            } else {
                // open frame: just the two rolls, advance by 2
                total += self.rolls[i] + self.rolls[i + 1];
                i += 2;
            }
        }
        Some(total)
    }
}
