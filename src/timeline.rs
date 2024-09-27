use std::time::Duration;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

macro_rules! tcode_full {
    ($h:tt:$m:tt:$s:tt:$f:tt:$nf:tt, $fr:expr) => {
        Timecode::new_with_framerate($h, $m, $s, $f, $nf, $fr)
    }
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

    pub fn hours(&self) -> &isize {
        &self.hours
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
        self.as_nanoseconds_with_framerate(&self.framerate)
    }

    pub fn as_nanoseconds_with_framerate(&self, fr: &Framerate) -> isize {
        let mut nanos: isize = 0;
        let framerate = if fr.as_f64() != 0.0 {
            fr.as_f64()
        } else {
            // If it's a timestamp, cancel the division.
            1.0
        };

        if let Framerate::Interpolated(_f) = fr {
            nanos += self.nanoframes / framerate as isize;
        }

        nanos += ((self.frames as f64 / framerate) * 1000000000.0) as isize;
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

    // Utils

    pub fn get_lerp_time_between(&self, begin: &Timecode, end: &Timecode) -> f64 {
        // TODO: There should've be a much efficient way to do this.
        // But i think it'll work for now...

        let t = self.as_nanoseconds();
        let a = begin.as_nanoseconds_with_framerate(&self.framerate);
        let b = end.as_nanoseconds_with_framerate(&self.framerate);

        let a_f64 = a as f64;
        let b_f64 = b as f64;
        let t_f64 = t as f64;

        let lerp = a_f64 + (b_f64 - a_f64) * (t_f64 / b_f64);
        let res = lerp / b_f64;

        res
    }
}

pub struct Timeline {
    time: Timecode,
}

impl Timeline {
    pub fn new(fr: Framerate) -> Self {
        Self {
            time: tcode_hmsf_framerate!(00:00:00:00, fr),
        }
    }

    pub fn time(&self) -> &Timecode {
        &self.time
    }

    pub fn add_by_duration(&mut self, d: Duration) {
        self.time.add_by_duration(d);
    }

    pub fn sub_by_duration(&mut self, d: Duration) {
        self.time.sub_by_duration(d);
    }

    pub fn set_by_duration(&mut self, d: Duration) {
        self.time.set_by_duration(d);
    }

    pub fn add_by_timestamp(&mut self, t: Timecode) {
        self.time.add_by_timestamp(t);
    }

    pub fn sub_by_timestamp(&mut self, t: Timecode) {
        self.time.sub_by_timestamp(t);
    }

    pub fn set_by_timestamp(&mut self, t: Timecode) {
        self.time.set_by_timestamp(t);
    }
}

#[derive(Debug)]
pub struct Timetree {
    tree: BTreeMap<isize, HourLeaf>,
}

impl Timetree {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
        }
    }

    // Create and get hour (HH:..)

    pub fn create_hour_with_isize(&mut self, hour: &isize) -> Option<&HourLeaf> {
        self.tree.insert(*hour, HourLeaf::default());
        self.get_hour_with_isize(hour)
    }

    pub fn create_hour_with_timestamp(&mut self, time: &Timecode) -> Option<&HourLeaf> {
        self.create_hour_with_isize(time.hours())
    }

    pub fn get_hour_with_isize(&self, hour: &isize) -> Option<&HourLeaf> {
        self.tree.get(hour)
    }

    pub fn get_hour_with_timestamp(&self, time: &Timecode) -> Option<&HourLeaf> {
        self.get_hour_with_isize(time.hours())
    }

    pub fn get_or_create_hour_with_isize(&mut self, hour: &isize) -> Option<&HourLeaf> {
        if self.get_hour_with_isize(hour).is_none() {
            return self.create_hour_with_isize(&hour);
        }

        self.get_hour_with_isize(hour)
    }

    pub fn get_or_create_hour_with_timestamp(&mut self, time: &Timecode) -> Option<&HourLeaf> {
        self.get_or_create_hour_with_isize(time.hours())
    }
}

#[derive(Debug, Default)]
pub struct HourLeaf {
    minute: BTreeMap<isize, MinuteLeaf>,
}

#[derive(Debug, Default)]
pub struct MinuteLeaf {
}