use std::time::Duration;

#[derive(Debug)]
pub enum Framerate {
    Timestamp,
    Fixed(f64),
    Interpolated(f64),
}

impl Framerate {
    pub fn as_string(&self) -> String {
        match self {
            Framerate::Timestamp => 0.0.to_string(),
            Framerate::Fixed(f) => f.to_string(),
            Framerate::Interpolated(f) => f.to_string(),
        }
    }

    pub fn as_f64(&self) -> f64 {
        match self {
            Framerate::Timestamp => 0.0,
            Framerate::Fixed(f) => *f,
            Framerate::Interpolated(f) => *f,
        }
    }

    pub fn is_timestamp(&self) -> bool {
        match self {
            Framerate::Timestamp => true,
            _ => false,
        }
    }

    pub fn is_interpolated(&self) -> bool {
    	match self {
    		Framerate::Interpolated(_s) => true,
    		_ => false,
    	}
    }
}

#[derive(Debug)]
pub struct Timecode {
    hours: isize,
    minutes: isize,
    seconds: isize,
    frames: f64,
    framerate: Framerate,
}

#[allow(unused_macros)]
macro_rules! tcode_hmsf {
    ($h:expr;$m:expr;$s:expr;$f:expr) => {
        Timecode::new($h, $m, $s, $f)
    };
}

#[allow(unused_macros)]
macro_rules! tcode_hmsf_framerate {
    ($h:expr;$m:expr;$s:expr;$f:expr, $fr:expr) => {
        Timecode::new_with_framerate($h, $m, $s, $f, $fr)
    };
}

#[allow(unused_macros)]
macro_rules! tcode_hms {
    ($h:expr;$m:expr;$s:expr) => {
        Timecode::new($h, $m, $s, 0.0)
    };
}

impl Timecode {
    pub fn new(h: isize, m: isize, s: isize, f: f64) -> Self {
        Timecode::new_with_framerate(h, m, s, f, Framerate::Timestamp)
    }

    pub fn new_with_framerate(h: isize, m: isize, s: isize, f: f64, fr: Framerate) -> Self {
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

    fn correct_overflow(&mut self) {
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

    fn correct_underflow(&mut self) {
        let framerate = self.framerate.as_f64();

        if framerate != 0.0 {
            while self.frames < 0.0 {
                self.seconds -= 1;
                self.frames += framerate;
            }
        }

        while self.seconds < 0 {
            self.minutes -= 1;
            self.seconds += 60;
        }

        while self.minutes < 0 {
            self.hours -= 1;
            self.minutes += 60;
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
        format!("{:?}:{:?}:{:?}", self.hours, self.minutes, self.seconds)
    }

    // Add/Next

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
        self.frames += secs * self.framerate.as_f64();

        self.correct_overflow();
    }

    // Sub/Back

    pub fn back_frame(&mut self) {
    	self.frames -= 1.0;
    	self.correct_underflow();
    }

    pub fn back_second(&mut self) {
    	self.seconds -= 1;
    	self.correct_underflow();
    }

    pub fn back_minute(&mut self) {
    	self.minutes -= 1;
    	self.correct_underflow();
    }

    pub fn back_hour(&mut self) {
    	self.hours -= 1;
    }

    pub fn sub_by_duration(&mut self, d: Duration) {
    	let secs = d.as_secs_f64();
        self.frames -= secs * self.framerate.as_f64();

        self.correct_underflow();
    }
}

pub struct Timeline {
    current_time: Timecode,
}
