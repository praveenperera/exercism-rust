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

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            hours: self.hours,
            minutes: self.minutes + minutes,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_minutes = (self.hours * 60) + self.minutes;

        let hours = (total_minutes / 60) % 24;
        let minutes = total_minutes % 60;

        // check if minutes are negative and rewind an hour
        let hours = if minutes < 0 { hours - 1 } else { hours };

        // deal with negative hours and minutes
        let minutes = (minutes + 60) % 60;
        let hours = (hours + 24) % 24;

        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
