use colored::*;
use std::{fmt::{self, Display}, time::Duration};

use crate::Position::{Bottom, Center, Top};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Clone, Copy)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {}, {})",
            self.position,
            self.size,
            self.content
                .truecolor(self.color.0, self.color.1, self.color.2))
    }
}

impl Event<'_> {
    pub fn notify(self) -> Notification {
        match self {
            Event::Remainder(s) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Bottom,
                content: s.to_string(),
            },
            Event::Registration(duration) => Notification {
                size: 30,
                color: (255, 2, 22),
                position: Top,
                content: format!("You have {} left before the registration ends", TimeLeft::from(duration))
            },
            Event::Appointment(s) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Center,
                content: s.to_string(),
            },
            Event::Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Top,
                content: format!("Enjoy your holiday"),
            },
        }
    }
}

struct TimeLeft {
    hours: u64,
    minutes: u64,
    seconds: u64,
}

impl From<Duration> for TimeLeft {
    fn from(duration: Duration) -> Self {
        let total_secs = duration.as_secs();
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;
        Self {
            hours,
            minutes,
            seconds,
        }
    }
}

impl Display for TimeLeft {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}H:{}M:{}S", self.hours, self.minutes, self.seconds)
    }
}