use std::time::Duration;

#[derive(Debug)]
pub enum Frame {
    Timestamp,
    Fixed(f64),
    Interpolated(f64),
}

impl Frame {
    pub fn as_string(&self) -> String {
        match self {
            Frame::Timestamp => 0.0.to_string(),
            Frame::Fixed(f) => f.to_string(),
            Frame::Interpolated(f) => f.to_string(),
        }
    }

    pub fn as_f64(&self) -> f64 {
        match self {
            Frame::Timestamp => 0.0,
            Frame::Fixed(f) => *f,
            Frame::Interpolated(f) => *f,
        }
    }

    pub fn is_timestamp(&self) -> bool {
        match self {
            Frame::Timestamp => true,
            _ => false,
        }
    }

    pub fn is_interpolated(&self) -> bool {
    	match self {
    		Frame::Interpolated(_s) => true,
    		_ => false,
    	}
    }
}

#[derive(Debug)]
pub struct Smpte {
    hours: isize,
    minutes: isize,
    seconds: isize,
    frames: f64,
    framerate: Frame,
}

#[allow(unused_macros)]
macro_rules! smpte_hmsf {
    ($h:expr;$m:expr;$s:expr;$f:expr) => {
        Smpte::new($h, $m, $s, $f)
    };
}

#[allow(unused_macros)]
macro_rules! smpte_hmsf_framerate {
    ($h:expr;$m:expr;$s:expr;$f:expr, $fr:expr) => {
        Smpte::new_with_framerate($h, $m, $s, $f, $fr)
    };
}

#[allow(unused_macros)]
macro_rules! smpte_hms {
    ($h:expr;$m:expr;$s:expr) => {
        Smpte::new($h, $m, $s, 0.0)
    };
}

impl Smpte {
    pub fn new(h: isize, m: isize, s: isize, f: f64) -> Self {
        Smpte::new_with_framerate(h, m, s, f, Frame::Timestamp)
    }

    pub fn new_with_framerate(h: isize, m: isize, s: isize, f: f64, fr: Frame) -> Self {
        let mut t = Self {
            hours: h,
            minutes: m,
            seconds: s,
            frames: f,
            framerate: fr,
        };

        t.correct_overflow();

        t
    }

    pub fn correct_overflow(&mut self) {
        let framerate = self.framerate.as_f64();

        if framerate != 0.0 {
            while self.frames >= framerate {
                self.seconds += 1;
                self.frames -= framerate;
            }
        }

        while self.seconds > 59 {
            self.minutes += 1;
            self.seconds -= 60;
        }

        while self.minutes > 59 {
            self.hours += 1;
            self.minutes -= 60;
        }
    }

    pub fn as_string(&self) -> String {
        format!(
            "{:?}:{:?}:{:?}:{:?} ({:?})",
            self.hours,
            self.minutes,
            self.seconds,
            self.frames,
            self.framerate.as_f64()
        )
    }

    pub fn hms_as_string(&self) -> String {
        format!("{:?}:{:?}:{:?}", self.hours, self.minutes, self.seconds,)
    }

    pub fn next_frame(&mut self) {
    	self.frames += 1.0;
    	self.correct_overflow();
    }

    pub fn next_second(&mut self) {
    	self.seconds += 1;
    	self.correct_overflow();
    }

    pub fn next_minute(&mut self) {
    	self.minutes += 1;
    	self.correct_overflow();
    }

    pub fn next_hour(&mut self) {
    	self.hours += 1;
    }

    pub fn add_by_duration(&mut self, d: Duration) {
        let secs = d.as_secs_f64();
        let frames = secs * self.framerate.as_f64();

        self.hours += (secs / 3600.0) as isize;
        self.minutes += (secs / 60.0) as isize;
        self.seconds += secs as isize;
        self.frames += frames;

        self.correct_overflow();
    }
}

pub struct Timeline {
    current_time: Smpte,
}
