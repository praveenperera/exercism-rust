use std::fmt;

#[derive(Debug)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: minutes + (hours * 60),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: self.minutes + minutes,
        }
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = (self.minutes / 60) % 24;
        let minutes = self.minutes % 60;

        // check if minutes are negative and rewind an hour
        let hours = if minutes < 0 { hours - 1 } else { hours };

        // deal with negative hours and minutes
        let minutes = (minutes + 60) % 60;
        let hours = (hours + 24) % 24;

        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
