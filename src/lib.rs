// Iteraion 3: reduces number of lines and memory required by calculating score as game progresses,
// instead of building game frames and waiting to calculate the score at the end
// This shorter version may not describe the rules of the game as explicitly as the prior one
// however.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    is_complete: bool,
    score: u16,
    frame: u8,
    first_pins: Option<u8>,
    bonus: (Option<u8>, Option<u8>),
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            is_complete: false,
            score: 0,
            frame: 1,
            first_pins: None,
            bonus: (None, None),
        }
    }

    pub fn roll(&mut self, pins: u8) -> Result<(), Error> {
        if self.is_complete {
            return Err(Error::GameComplete);
        }

        if pins + self.first_pins.unwrap_or(0) > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.score += (pins * (1 + self.bonus.0.unwrap_or(0))) as u16;
        self.bonus = (self.bonus.1, None);

        if self.frame < 11 {
            if pins + self.first_pins.unwrap_or(0) == 10 {
                let bonus = if self.frame < 10 { 1 } else { 0 };
                if pins == 10 {
                    self.bonus = (Some(bonus + self.bonus.0.unwrap_or(0)), Some(bonus));
                } else {
                    self.bonus.0 = Some(bonus + self.bonus.0.unwrap_or(0));
                }
            }
            if self.first_pins.is_some() || pins == 10 {
                self.frame += 1;
                self.first_pins = None;
            } else {
                self.first_pins = Some(pins);
            }
        } else if pins < 10 {
            self.first_pins = Some(pins);
        }

        self.is_complete = self.frame == 11 && self.bonus == (None, None);

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_complete {
            Some(self.score)
        } else {
            None
        }
    }
}
