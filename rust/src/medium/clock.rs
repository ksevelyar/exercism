use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let normalized_hours = hours % 24;
        Clock {
            hours: normalized_hours,
            minutes,
        }
    }

    fn state_in_minutes(&self) -> i32 {
        self.hours * 60 + self.minutes
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes_per_day = 24 * 60;
        let new_state = (self.state_in_minutes() + minutes) % minutes_per_day;

        let new_state_minutes = new_state % 60;
        let new_state_hours = new_state / 60;

        Clock {
            hours: new_state_hours,
            minutes: new_state_minutes,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
