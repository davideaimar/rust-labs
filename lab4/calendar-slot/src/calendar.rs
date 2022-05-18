use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use crate::timeslot::Timeslot;

#[derive(Debug)]
pub struct Calendar {
    schedule: Vec<Timeslot>,
    bounds: Timeslot
}

impl Calendar {
    pub fn new(schedule: Vec<Timeslot>, bounds: Timeslot) -> Calendar {
        Calendar {
            schedule,
            bounds
        }
    }
    pub fn from_file(path: &str) -> Result<Calendar, Box<dyn std::error::Error>> {
        let f = File::open(path)?;
        let f = BufReader::new(f);
        let mut slots = Vec::new();
        let mut lines = f.lines();
        let bounds = Timeslot::from_str(lines.next().unwrap()?.as_str())?;

        for line in lines {
            let line = line?;
            let slot = Timeslot::from_str(&line)?;
            slots.push(slot);
        }

        Ok(Calendar::new(
            slots,
            bounds
        ))
    }
    pub fn free_slots_with(&self, cal_b: &Calendar, duration: u32) -> Vec<Timeslot> {
        let mut free_slots: Vec<Timeslot> = Vec::new();

        let mut cal_a_iter = self.schedule.iter();
        let mut cal_b_iter = cal_b.schedule.iter();

        let mut next_a = cal_a_iter.next();
        let mut next_b = cal_b_iter.next();

        let mut free_a = if let Some(value) = next_a {
            Timeslot::new(self.bounds.start, value.start)
        } else {
            self.bounds
        };

        let mut free_b = if let Some(value) = next_b {
            Timeslot::new(cal_b.bounds.start, value.start)
        } else {
            cal_b.bounds
        };

        loop {
            let free_intersection = free_a.intersect_with(&free_b);
            if free_intersection.get_duration() >= duration {
                free_slots.push(free_intersection);
            }
            if free_b.end < free_a.start && free_b.end < cal_b.bounds.end {
                let old_end = next_b.unwrap().end;
                next_b = cal_b_iter.next();
                if let Some(value) = next_b {
                    free_b = Timeslot::new(old_end, value.start);
                }
                else {
                    free_b = Timeslot::new(old_end, cal_b.bounds.end);
                }
            } else if free_a.end < self.bounds.end {
                let old_end = next_a.unwrap().end;
                next_a = cal_a_iter.next();
                if let Some(value) = next_a {
                    free_a = Timeslot::new(old_end, value.start);
                }
                else {
                    free_a = Timeslot::new(old_end, self.bounds.end);
                }
            }
            else {
                break;
            }
        }

        free_slots
    }

}

