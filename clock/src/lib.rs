use std::fmt::Display;

#[derive(Debug)]
pub struct Clock(i32);

const CLK_HOURS: i32 = 60;
const CLK_DAY: i32 = 24 * CLK_HOURS;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Reduce dates as minutes, module a full day to rebound between
        // (-1 day, +1 day), add a day to shift bounds to [0, +2 day)
        // and remodulo a day to [0, +1 day)
        Self((((hours * CLK_HOURS + minutes) % CLK_DAY) + CLK_DAY) % CLK_DAY)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // Just add the given minutes to the current minutes and forget the hours
        // as these are included in the minutes and use the constructor func
        // to calc fix offset
        Self::new(0, self.0 + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:02}:{:02}", self.0 / 60, self.0 % 60))
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
