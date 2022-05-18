use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Sub};

pub struct Clock{
    minutes: i32,
}

fn format(minutes: i32) -> String {
    let mut hours = 0;
    let mut minutes = minutes;
    while minutes < 0 {
        hours -= 1;
        minutes += 60;
    }
    while hours < 0 {
        hours += 24;
    }
    hours += minutes / 60;
    format!("{:02}:{:02}", hours % 24, minutes % 60)
}

impl Display for Clock{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", format(self.minutes))
    }
}

impl PartialEq<Self> for Clock {
    fn eq(&self, other: &Self) -> bool {
        return format(self.minutes) == format(other.minutes);
    }
}

impl Debug for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format(self.minutes))
    }
}

impl Add for Clock {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            minutes: self.minutes + rhs.minutes,
        }
    }
}

impl Sub for Clock {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            minutes: self.minutes - rhs.minutes,
        }
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes = minutes + hours*60;
        Clock{ minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock{ minutes: self.minutes + minutes }
    }
}
