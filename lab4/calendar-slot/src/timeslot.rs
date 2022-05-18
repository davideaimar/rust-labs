use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Timeslot {
    pub start: u32,
    pub end: u32
}

impl Timeslot {
    pub fn new(start: u32, end: u32) -> Timeslot {
        Timeslot {
            start,
            end
        }
    }

    pub fn get_duration(&self) -> u32 {
        self.end - self.start
    }

    pub fn intersect_with(&self, other: &Timeslot) -> Timeslot {
        let start = std::cmp::max(self.start, other.start);
        let end = std::cmp::min(self.end, other.end);
        if start < end {
            Timeslot {
                start,
                end
            }
        } else {
            Timeslot::new(0, 0)
        }
    }
}

impl Display for Timeslot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02} {:02}:{:02}", self.start/60, self.start%60, self.end/60, self.end%60)
    }
}

impl FromStr for Timeslot {
    type Err = Box<dyn Error>;

    fn from_str(slot: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = slot.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(format!("Invalid timeslot {}", slot).into());
        }

        let start_parts = parts[0].split(':').collect::<Vec<&str>>();
        let end_parts = parts[1].split(':').collect::<Vec<&str>>();
        if start_parts.len() != 2 || end_parts.len() != 2 {
            return Err(format!("Invalid timeslot {}", slot).into());
        }

        let start_h = start_parts[0].parse::<u32>()?;
        let start_m = start_parts[1].parse::<u32>()?;
        let end_h = end_parts[0].parse::<u32>()?;
        let end_m = end_parts[1].parse::<u32>()?;

        if end_h > 23 || end_m > 59 || start_h > 23 || start_m > 59 {
            return Err(format!("Invalid timeslot {}", slot).into());
        }

        Ok(Timeslot {
            start: start_h*60 + start_m,
            end: end_h*60 + end_m
        })
    }
}