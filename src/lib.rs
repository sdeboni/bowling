// Iteration 2:
// 1. reassigning variable name to remove self. inspired by potatosalad's solution
// 2. replaced panic with invariant
// 3. simplified definitions of frame is_strike, spare and complete
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
        assert!(pins < 11);
        let roll = &mut self.roll;

        assert!(roll.0.is_none() || roll.1.is_none());

        if roll.0.is_none() {
            assert!(roll.1.is_none());
            roll.0 = Some(pins);
        } else if roll.1.is_none() {
            assert!(roll.0.is_some());
            if roll.0.unwrap() + pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
            roll.1 = Some(pins);
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
        self.roll.0.map_or_else(|| false, |v| v == 10)
    }

    fn is_spare(&self) -> bool {
        self.pins() == 10 && !self.is_strike()
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
        let frames = &mut self.frames;
        let fill = &mut self.fill;

        if pins > 10 {
            Err(Error::NotEnoughPinsLeft)
        } else {
            match frames.iter_mut().find(|frame| !frame.is_complete()) {
                Some(frame) => frame.roll(pins),
                None => {
                    if frames[9].is_strike() {
                        if fill.0.is_none() {
                            fill.0 = Some(pins);
                        } else if fill.1.is_none() {
                            if fill.0.unwrap() < 10 && fill.0.unwrap() + pins > 10 {
                                return Err(Error::NotEnoughPinsLeft);
                            } else {
                                fill.1 = Some(pins);
                            }
                        } else {
                            return Err(Error::GameComplete);
                        }
                    } else if frames[9].is_spare() && fill.0.is_none() {
                        fill.0 = Some(pins);
                    } else {
                        return Err(Error::GameComplete);
                    }
                    Ok(())
                }
            }
        }
    }

    fn is_complete(&self) -> bool {
        let frames = &self.frames;
        let fill = &self.fill;

        if frames.iter().all(|frame| frame.is_complete()) {
            if frames[9].is_strike() {
                fill.0.is_some() && fill.1.is_some()
            } else if frames[9].is_spare() {
                fill.0.is_some()
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete() {
            return None;
        }

        let frames = &self.frames;
        let fill = &self.fill;

        let mut score = 0_u16;

        // Score first 8 frames
        for i in 0..8 {
            score += frames[i].pins() as u16;
            if frames[i].is_strike() {
                score += frames[i + 1].pins() as u16;
                if frames[i + 1].is_strike() {
                    score += frames[i + 2].roll.0.unwrap() as u16;
                }
            } else if frames[i].is_spare() {
                score += frames[i + 1].roll.0.unwrap() as u16;
            }
        }

        // Score ninth frame
        score += frames[8].pins() as u16;
        if frames[8].is_strike() {
            score += frames[9].pins() as u16;
            if frames[9].is_strike() {
                score += fill.0.unwrap() as u16;
            }
        } else if frames[8].is_spare() {
            score += frames[9].roll.0.unwrap() as u16;
        }

        // Score last frame
        score += frames[9].pins() as u16;
        if frames[9].is_strike() {
            score += fill.0.unwrap() as u16 + fill.1.unwrap() as u16;
        } else if frames[9].is_spare() {
            score += fill.0.unwrap() as u16;
        }

        Some(score)
    }
}
