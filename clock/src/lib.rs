use std::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: hours,
            minutes: minutes,
        }
    }

    pub fn to_string(&self) -> String {
        let hours = match self.hours {
            24 => 0,
            _ => self.hours,
        };

        format!("{:02}:{:02}", hours, self.minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02})", self.hours, self.minutes)
    }
}
