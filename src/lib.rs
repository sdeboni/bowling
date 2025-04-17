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
    frame_number: u8,
    first_frame_roll_pins: Option<u8>,
    roll_bonus: (Option<u8>, Option<u8>),
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            is_complete: false,
            score: 0,
            frame_number: 1,
            first_frame_roll_pins: None,
            roll_bonus: (None, None),
        }
    }

    pub fn roll(&mut self, pins: u8) -> Result<(), Error> {
        if self.is_complete {
            return Err(Error::GameComplete);
        }

        if pins + self.first_frame_roll_pins.unwrap_or(0) > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.score += (pins * (1 + self.roll_bonus.0.unwrap_or(0))) as u16;
        self.roll_bonus = (self.roll_bonus.1, None);

        if self.frame_number < 11 {
            if pins + self.first_frame_roll_pins.unwrap_or(0) == 10 {
                let bonus = if self.frame_number < 10 { 1 } else { 0 };
                if pins == 10 {
                    self.roll_bonus = (Some(bonus + self.roll_bonus.0.unwrap_or(0)), Some(bonus));
                } else {
                    self.roll_bonus.0 = Some(bonus + self.roll_bonus.0.unwrap_or(0));
                }
            }
            if self.first_frame_roll_pins.is_some() || pins == 10 {
                self.frame_number += 1;
                self.first_frame_roll_pins = None;
            } else {
                self.first_frame_roll_pins = Some(pins);
            }
        } else if pins < 10 {
            self.first_frame_roll_pins = Some(pins);
        }

        self.is_complete = self.frame_number == 11 && self.roll_bonus == (None, None);

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
