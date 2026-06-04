#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
      pub fn new(hours: i32, minutes: i32) -> Self {
        // todo!("Construct a new Clock from {hours} hours and {minutes} minutes");
        let total_minutes = hours * 60 + minutes;
        Clock::normalize_clock_from_minutes(total_minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // todo!("Add {minutes} minutes to existing Clock time");
        let total_minutes = self.hours * 60 + (self.minutes + minutes);

        let normalized = total_minutes.rem_euclid(24 * 60);

        Clock::normalize_clock_from_minutes(normalized)
    }
    fn normalize_clock_from_minutes(minutes: i32) -> Clock {
        let normalized = minutes.rem_euclid(24 * 60);

        Clock {
            hours: normalized / 60,
            minutes: normalized % 60,
        }
    }

}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}



