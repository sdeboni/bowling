#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Clone, Copy)]
struct Frame {
    roll: (Option<u8>, Option<u8>),
}

impl Frame {
    fn roll(&mut self, pins: u8) -> Result<(), Error> {
        assert!(pins <= 10);

        if self.roll.0.is_none() {
            assert!(self.roll.1.is_none());
            self.roll.0 = Some(pins);
        } else if self.roll.1.is_none() {
            if self.roll.0.unwrap() + pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
            self.roll.1 = Some(pins);
        } else {
            panic!("cannot add roll to frame; frame is complete");
        }
        Ok(())
    }

    fn pins(&self) -> u8 {
        self.roll.0.unwrap_or(0) + self.roll.1.unwrap_or(0)
    }

    fn is_complete(&self) -> bool {
        self.is_strike() || (self.roll.0.is_some() && self.roll.1.is_some())
    }

    fn is_strike(&self) -> bool {
        if self.roll.0.map_or_else(|| false, |v| v == 10) {
            assert!(self.roll.1.is_none());
            true
        } else {
            false
        }
    }

    fn is_spare(&self) -> bool {
        if self.is_complete() {
            self.roll.0.unwrap() + self.roll.1.unwrap() == 10
        } else {
            false
        }
    }
}

pub struct BowlingGame {
    frames: [Frame; 10],
    fill: (Option<u8>, Option<u8>),
}

impl BowlingGame {
    pub fn new() -> Self {
        let empty_frame = Frame { roll: (None, None) };

        BowlingGame {
            frames: [empty_frame; 10],
            fill: (None, None),
        }
    }

    pub fn roll(&mut self, pins: u8) -> Result<(), Error> {
        if pins > 10 {
            Err(Error::NotEnoughPinsLeft)
        } else {
            match self.frames.iter_mut().find(|frame| !frame.is_complete()) {
                Some(frame) => frame.roll(pins),
                None => {
                    if self.frames[9].is_strike() {
                        if self.fill.0.is_none() {
                            self.fill.0 = Some(pins);
                        } else if self.fill.1.is_none() {
                            if self.fill.0.unwrap() < 10 && self.fill.0.unwrap() + pins > 10 {
                                return Err(Error::NotEnoughPinsLeft);
                            } else {
                                self.fill.1 = Some(pins);
                            }
                        } else {
                            return Err(Error::GameComplete);
                        }
                    } else if self.frames[9].is_spare() && self.fill.0.is_none() {
                        self.fill.0 = Some(pins);
                    } else {
                        return Err(Error::GameComplete);
                    }
                    Ok(())
                }
            }
        }
    }

    fn is_complete(&self) -> bool {
        if self.frames.iter().all(|frame| frame.is_complete()) {
            if self.frames[9].is_strike() {
                self.fill.0.is_some() && self.fill.1.is_some()
            } else if self.frames[9].is_spare() {
                self.fill.0.is_some()
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_complete() {
            let mut score = 0_u16;

            for i in 0..8 {
                score += self.frames[i].pins() as u16;
                if self.frames[i].is_strike() {
                    score += self.frames[i + 1].pins() as u16;
                    if self.frames[i + 1].is_strike() {
                        score += self.frames[i + 2].roll.0.unwrap() as u16;
                    }
                } else if self.frames[i].is_spare() {
                    score += self.frames[i + 1].roll.0.unwrap() as u16;
                }
            }

            score += self.frames[8].pins() as u16;
            if self.frames[8].is_strike() {
                score += self.frames[9].pins() as u16;
                if self.frames[9].is_strike() {
                    score += self.fill.0.unwrap() as u16;
                }
            } else if self.frames[8].is_spare() {
                score += self.frames[9].roll.0.unwrap() as u16;
            }

            score += self.frames[9].pins() as u16;
            if self.frames[9].is_strike() {
                score += self.fill.0.unwrap() as u16 + self.fill.1.unwrap() as u16;
            } else if self.frames[9].is_spare() {
                score += self.fill.0.unwrap() as u16;
            }
            Some(score)
        } else {
            None
        }
    }
}
