//! # Timer
//!
//! A timer that mimics iOS's timer.
//!
//! ## Usage
//!
//! - Use `Timer::new(<duration>)` to initialise a new timer instance. `<duration>` is a
//! `chrono::Duration`. The timer is paused at the duration you specified and will **not**
//! run until you call `.resume()` or `.pause_or_resume()`.
//! - While running, call `.pause_or_resume()`, `.pause()` or `.resume()` to pause or resume.
//! - When you want to stop (reset), call `.stop()`, which resets the timer and returns
//!   [`TimerData`](struct.TimerData.html)

use chrono::{DateTime, Duration, Local};

#[derive(Debug, Clone)]
pub struct TimerData {
    pub total: Duration,
    pub remaining: Duration,
    pub start_moments: Vec<DateTime<Local>>, // moments at which the timer resumes; the first is the start monent
    pub pause_moments: Vec<DateTime<Local>>, // moments at which the timer is paused; the last is the stop moment
}

impl TimerData {
    fn new(duration: Duration) -> Self {
        Self {
            total: duration,
            remaining: duration,
            start_moments: Vec::new(),
            pause_moments: Vec::new(),
        }
    }
    pub fn start(&self) -> DateTime<Local> {
        self.start_moments[0]
    }
    pub fn stop(&self) -> DateTime<Local> {
        self.pause_moments[self.pause_moments.len() - 1]
    }
    pub fn duration_expected(&self) -> Duration {
        self.total
    }
    pub fn duration_actual(&self) -> Duration {
        self.stop() - self.start()
    }
}

/// A countdown timer
#[derive(Clone, Debug)]
pub struct Timer {
    pub paused: bool,
    pub data: TimerData,
}

impl Timer {
    /// Returns stopwatch reset to zero
    pub fn new(duration: Duration) -> Self {
        Self {
            paused: true, // finished by default; start by explicitly calling `.resume()`
            data: TimerData::new(duration),
        }
    }
    /// Read the timer. Returns the duration passed.
    pub fn read(&self) -> Duration {
        if self.paused {
            self.data.remaining
        } else {
            self.data.remaining - (Local::now() - self.last_start())
        }
    }
    /// Pause or resume the timer. (If paused, resume, and vice versa.)
    pub fn pause_or_resume(&mut self) {
        self.pause_or_resume_at(Local::now());
    }

    pub fn pause_or_resume_at(&mut self, moment: DateTime<Local>) {
        if self.paused {
            self.resume_at(moment);
        } else {
            self.pause_at(moment);
        }
    }

    /// Pause the timer (suggest using `pause_or_resume` instead.)
    pub fn pause(&mut self) {
        self.pause_at(Local::now());
    }

    pub fn pause_at(&mut self, moment: DateTime<Local>) {
        self.data.pause_moments.push(moment);
        self.data.remaining = self.data.remaining - (moment - self.last_start());
        self.paused = true;
    }
    /// Resume the timer (suggest using `pause_or_resume` instead.)
    pub fn resume(&mut self) {
        self.resume_at(Local::now());
    }

    pub fn resume_at(&mut self, moment: DateTime<Local>) {
        self.data.start_moments.push(moment);
        self.paused = false;
    }

    /// Stop the timer, return the data, and reset the timer with the previously set duration.
    pub fn stop(&mut self) -> TimerData {
        self.stop_at(Local::now())
    }

    pub fn stop_at(&mut self, moment: DateTime<Local>) -> TimerData {
        self.data.pause_moments.push(moment);
        let duration = self.data.total;
        let data = std::mem::replace(&mut self.data, TimerData::new(duration));
        data
    }

    fn last_start(&self) -> DateTime<Local> {
        self.data.start_moments[self.data.start_moments.len() - 1]
    }
}
