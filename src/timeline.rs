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
    frames: isize,
    nanoframes: isize,
    framerate: Framerate,
}

#[allow(unused_macros)]
macro_rules! tcode_hmsf {
    ($h:tt:$m:tt:$s:tt:$f:tt) => {
        Timecode::new($h, $m, $s, $f)
    };
}

#[allow(unused_macros)]
macro_rules! tcode_hmsf_framerate {
    ($h:tt:$m:tt:$s:tt:$f:tt, $fr:expr) => {
        Timecode::new_with_framerate($h, $m, $s, $f, 0, $fr)
    };
}

#[allow(unused_macros)]
macro_rules! tcode_hms {
    ($h:tt:$m:tt:$s:tt) => {
        Timecode::new($h, $m, $s, 0)
    };
}

impl Timecode {
    pub fn new(h: isize, m: isize, s: isize, f: isize) -> Self {
        Timecode::new_with_framerate(h, m, s, f, 0, Framerate::Timestamp)
    }

    pub fn new_with_framerate(
        h: isize,
        m: isize,
        s: isize,
        f: isize,
        nf: isize,
        fr: Framerate,
    ) -> Self {
        let mut t = Self {
            hours: h,
            minutes: m,
            seconds: s,
            frames: f,
            nanoframes: nf,
            framerate: fr,
        };

        t.correct_overflow();

        t
    }

    fn correct_overflow(&mut self) {
        let framerate = self.framerate.as_f64();

        if framerate != 0.0 {
            while self.nanoframes > 999999999 {
                self.frames += 1;
                self.nanoframes -= 1000000000;
            }

            while self.frames >= framerate as isize {
                self.seconds += 1;
                self.frames -= framerate as isize;
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
            while self.nanoframes < 0 {
                self.frames -= 1;
                self.nanoframes += 1000000000;
            }

            while self.frames < 0 {
                self.seconds -= 1;
                self.frames += framerate as isize;
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
            "{:02}:{:02}:{:02}:{:02} ({:?})",
            self.hours,
            self.minutes,
            self.seconds,
            self.frames,
            self.framerate.as_f64()
        )
    }

    pub fn full_as_string(&self) -> String {
        format!(
            "{:02}:{:02}:{:02}:{:02}:{:09} ({:?})",
            self.hours,
            self.minutes,
            self.seconds,
            self.frames,
            self.nanoframes,
            self.framerate.as_f64()
        )
    }

    pub fn hms_as_string(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
    }

    // Add/Next

    #[inline]
    pub fn next_frame(&mut self) {
        self.frames += 1;
        self.correct_overflow();
    }

    #[inline]
    pub fn next_second(&mut self) {
        self.seconds += 1;
        self.correct_overflow();
    }

    #[inline]
    pub fn next_minute(&mut self) {
        self.minutes += 1;
        self.correct_overflow();
    }

    #[inline]
    pub fn next_hour(&mut self) {
        self.hours += 1;
    }

    pub fn add_by_duration(&mut self, d: Duration) {
        let nanos = d.as_nanos() as isize;
        self.nanoframes += nanos * self.framerate.as_f64() as isize;

        self.correct_overflow();
    }

    pub fn add_by_timestamp(&mut self, t: Timecode) {
        let nanos = t.as_nanoseconds();
        self.nanoframes += nanos * self.framerate.as_f64() as isize;

        self.correct_overflow();
    }

    // Sub/Back

    #[inline]
    pub fn back_frame(&mut self) {
        self.frames -= 1;
        self.correct_underflow();
    }

    #[inline]
    pub fn back_second(&mut self) {
        self.seconds -= 1;
        self.correct_underflow();
    }

    #[inline]
    pub fn back_minute(&mut self) {
        self.minutes -= 1;
        self.correct_underflow();
    }

    #[inline]
    pub fn back_hour(&mut self) {
        self.hours -= 1;
    }

    pub fn sub_by_duration(&mut self, d: Duration) {
        let nanos = d.as_nanos() as isize;
        self.nanoframes -= nanos * self.framerate.as_f64() as isize;

        self.correct_underflow();
    }

    pub fn sub_by_timestamp(&mut self, t: Timecode) {
        let nanos = t.as_nanoseconds();
        self.nanoframes -= nanos * self.framerate.as_f64() as isize;

        self.correct_underflow();
    }

    pub fn reset(&mut self) {
        self.nanoframes = 0;
        self.frames = 0;
        self.seconds = 0;
        self.minutes = 0;
        self.hours = 0;
    }

    pub fn set_by_duration(&mut self, d: Duration) {
        self.reset();
        self.add_by_duration(d);
    }

    pub fn set_by_timestamp(&mut self, t: Timecode) {
        self.reset();
        self.add_by_timestamp(t);
    }

    pub fn as_nanoseconds(&self) -> isize {
        let mut nanos: isize = 0;
        let framerate = if self.framerate.as_f64() != 0.0 {
            self.framerate.as_f64()
        } else {
            // If it's a timestamp, cancel the division.
            1.0
        };

        nanos += self.nanoframes / framerate as isize;
        nanos += (self.frames / framerate as isize) * 1_000_000;
        nanos += self.seconds * (1e+9 as isize);
        nanos += self.minutes * (6e+10 as isize);
        nanos += self.hours * (3.6e+12 as isize);

        nanos
    }

    // Checks

    pub fn is_equals_to_hmsf(&self, t: &Timecode) -> bool {
        self.frames == t.frames
            && self.seconds == t.seconds
            && self.minutes == t.minutes
            && self.hours == t.hours
    }
}

pub struct Timeline {
    time: Timecode,
}
