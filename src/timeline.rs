use crate::Tween;
use anymap::AnyMap;
use core::any::Any;
use std::collections::BTreeMap;
use std::time::Duration;

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
            Framerate::Fixed(f) | Framerate::Interpolated(f) => f.to_string(),
        }
    }

    pub fn as_f64(&self) -> f64 {
        match self {
            Framerate::Timestamp => 0.0,
            Framerate::Fixed(f) | Framerate::Interpolated(f) => *f,
        }
    }

    pub fn is_timestamp(&self) -> bool {
        matches!(self, Framerate::Timestamp)
    }

    pub fn is_interpolated(&self) -> bool {
        matches!(self, Framerate::Interpolated(_s))
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

#[allow(unused_macros)]
macro_rules! tcode_full {
    ($h:tt:$m:tt:$s:tt:$f:tt:$nf:tt, $fr:expr) => {
        Timecode::new_with_framerate($h, $m, $s, $f, $nf, $fr)
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

    #[inline]
    pub fn hours(&self) -> &isize {
        &self.hours
    }

    #[inline]
    pub fn minutes(&self) -> &isize {
        &self.minutes
    }

    #[inline]
    pub fn seconds(&self) -> &isize {
        &self.seconds
    }

    #[inline]
    pub fn frames(&self) -> &isize {
        &self.frames
    }

    #[inline]
    pub fn nanoframes(&self) -> &isize {
        &self.nanoframes
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

        lerp / b_f64
    }
}

#[derive(Debug)]
pub struct Timeline {
    time: Timecode,
    sequences: AnyMap,
}

impl Timeline {
    pub fn new(fr: Framerate) -> Self {
        Self {
            time: tcode_hmsf_framerate!(00:00:00:00, fr),
            sequences: AnyMap::new(),
        }
    }

    pub fn new_sequence<T: Tween + 'static>(&mut self) -> Option<&mut Sequence<T>> {
        if self.sequences.get::<Vec<Sequence<T>>>().is_none() {
            self.sequences.insert(Vec::<Sequence<T>>::new());
        }

        let mut seq_list = self.sequences.get_mut::<Vec<Sequence<T>>>().unwrap();
        seq_list.push(Sequence::<T>::new());

        seq_list.last_mut()
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
pub struct Sequence<T: Tween> {
    tree: BTreeMap<isize, HourLeaf<T>>,
}

impl<T: Tween> Default for Sequence<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Tween> Sequence<T> {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
        }
    }

    // Create and get hour

    #[inline]
    pub fn create_hour_with_isize(&mut self, hour: &isize) -> Option<&mut HourLeaf<T>> {
        self.tree.insert(*hour, HourLeaf::<T>::new());
        self.get_hour_with_isize(hour)
    }

    #[inline]
    pub fn create_hour_with_timestamp(&mut self, time: &Timecode) -> Option<&mut HourLeaf<T>> {
        self.create_hour_with_isize(time.hours())
    }

    #[inline]
    pub fn get_hour_with_isize(&mut self, hour: &isize) -> Option<&mut HourLeaf<T>> {
        self.tree.get_mut(hour)
    }

    #[inline]
    pub fn get_hour_with_timestamp(&mut self, time: &Timecode) -> Option<&mut HourLeaf<T>> {
        self.get_hour_with_isize(time.hours())
    }

    #[inline]
    pub fn get_or_create_hour_with_isize(&mut self, hour: &isize) -> Option<&mut HourLeaf<T>> {
        if self.get_hour_with_isize(hour).is_none() {
            return self.create_hour_with_isize(hour);
        }

        self.get_hour_with_isize(hour)
    }

    #[inline]
    pub fn get_or_create_hour_with_timestamp(
        &mut self,
        time: &Timecode,
    ) -> Option<&mut HourLeaf<T>> {
        self.get_or_create_hour_with_isize(time.hours())
    }

    /// # Panics
    ///
    /// TODO!
    pub fn add_keyframe_at_timestamp(
        &mut self,
        key: Keyframe<T>,
        time: &Timecode,
    ) -> Option<&mut Keyframe<T>> {
        // TODO: Make it so it returns 'None' instead of panicking.

        let hour: &mut HourLeaf<T> = self.get_or_create_hour_with_timestamp(time).unwrap();
        let minute: &mut MinuteLeaf<T> = hour.get_or_create_minute_with_timestamp(time).unwrap();
        let second: &mut SecondLeaf<T> = minute.get_or_create_second_with_timestamp(time).unwrap();
        let frame: &mut FrameLeaf<T> = second.get_or_create_frame_with_timestamp(time).unwrap();

        frame.add_keyframe_at_timestamp(time, key)
    }

    /// # Panics
    ///
    /// TODO!
    pub fn get_keyframe_at_timestamp(&mut self, time: &Timecode) -> Option<&mut Keyframe<T>> {
        // TODO: Make it so it returns 'None' instead of panicking.

        let hour: &mut HourLeaf<T> = self.get_hour_with_timestamp(time).unwrap();
        let minute: &mut MinuteLeaf<T> = hour.get_minute_with_timestamp(time).unwrap();
        let second: &mut SecondLeaf<T> = minute.get_second_with_timestamp(time).unwrap();
        let frame: &mut FrameLeaf<T> = second.get_frame_with_timestamp(time).unwrap();

        frame.get_keyframe_at_timestamp(time)
    }

    /// # Panics
    ///
    /// TODO!
    pub fn add_keyframes_at_timestamp(&mut self, keyframes: Vec<(Keyframe<T>, &Timecode)>) {
        for k in keyframes {
            self.add_keyframe_at_timestamp(k.0, k.1);
        }
    }
}

#[derive(Debug)]
pub struct HourLeaf<T: Tween> {
    minutes: BTreeMap<isize, MinuteLeaf<T>>,
}

impl<T: Tween> Default for HourLeaf<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Tween> HourLeaf<T> {
    pub fn new() -> Self {
        Self {
            minutes: BTreeMap::new(),
        }
    }

    // Create and get minute

    #[inline]
    pub fn create_minute_with_isize(&mut self, minute: &isize) -> Option<&mut MinuteLeaf<T>> {
        self.minutes.insert(*minute, MinuteLeaf::<T>::new());
        self.get_minute_with_isize(minute)
    }

    #[inline]
    pub fn create_minute_with_timestamp(&mut self, time: &Timecode) -> Option<&mut MinuteLeaf<T>> {
        self.create_minute_with_isize(time.minutes())
    }

    #[inline]
    pub fn get_minute_with_isize(&mut self, minute: &isize) -> Option<&mut MinuteLeaf<T>> {
        self.minutes.get_mut(minute)
    }

    #[inline]
    pub fn get_minute_with_timestamp(&mut self, time: &Timecode) -> Option<&mut MinuteLeaf<T>> {
        self.get_minute_with_isize(time.minutes())
    }

    #[inline]
    pub fn get_or_create_minute_with_isize(
        &mut self,
        minute: &isize,
    ) -> Option<&mut MinuteLeaf<T>> {
        if self.get_minute_with_isize(minute).is_none() {
            return self.create_minute_with_isize(minute);
        }

        self.get_minute_with_isize(minute)
    }

    #[inline]
    pub fn get_or_create_minute_with_timestamp(
        &mut self,
        time: &Timecode,
    ) -> Option<&mut MinuteLeaf<T>> {
        self.get_or_create_minute_with_isize(time.minutes())
    }
}

#[derive(Debug)]
pub struct MinuteLeaf<T: Tween> {
    seconds: BTreeMap<isize, SecondLeaf<T>>,
}

impl<T: Tween> Default for MinuteLeaf<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Tween> MinuteLeaf<T> {
    pub fn new() -> Self {
        Self {
            seconds: BTreeMap::new(),
        }
    }

    // Create and get second

    #[inline]
    pub fn create_second_with_isize(&mut self, second: &isize) -> Option<&mut SecondLeaf<T>> {
        self.seconds.insert(*second, SecondLeaf::<T>::new());
        self.get_second_with_isize(second)
    }

    #[inline]
    pub fn create_second_with_timestamp(&mut self, time: &Timecode) -> Option<&mut SecondLeaf<T>> {
        self.create_second_with_isize(time.seconds())
    }

    #[inline]
    pub fn get_second_with_isize(&mut self, second: &isize) -> Option<&mut SecondLeaf<T>> {
        self.seconds.get_mut(second)
    }

    #[inline]
    pub fn get_second_with_timestamp(&mut self, time: &Timecode) -> Option<&mut SecondLeaf<T>> {
        self.get_second_with_isize(time.seconds())
    }

    #[inline]
    pub fn get_or_create_second_with_isize(
        &mut self,
        second: &isize,
    ) -> Option<&mut SecondLeaf<T>> {
        if self.get_second_with_isize(second).is_none() {
            return self.create_second_with_isize(second);
        }

        self.get_second_with_isize(second)
    }

    #[inline]
    pub fn get_or_create_second_with_timestamp(
        &mut self,
        time: &Timecode,
    ) -> Option<&mut SecondLeaf<T>> {
        self.get_or_create_second_with_isize(time.seconds())
    }
}

#[derive(Debug)]
pub struct SecondLeaf<T: Tween> {
    frames: BTreeMap<isize, FrameLeaf<T>>,
}

impl<T: Tween> Default for SecondLeaf<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Tween> SecondLeaf<T> {
    pub fn new() -> Self {
        Self {
            frames: BTreeMap::new(),
        }
    }

    // Create and get frame

    #[inline]
    pub fn create_frame_with_isize(&mut self, frame: &isize) -> Option<&mut FrameLeaf<T>> {
        self.frames.insert(*frame, FrameLeaf::<T>::new());
        self.get_frame_with_isize(frame)
    }

    #[inline]
    pub fn create_frame_with_timestamp(&mut self, time: &Timecode) -> Option<&mut FrameLeaf<T>> {
        self.create_frame_with_isize(time.frames())
    }

    #[inline]
    pub fn get_frame_with_isize(&mut self, frame: &isize) -> Option<&mut FrameLeaf<T>> {
        self.frames.get_mut(frame)
    }

    #[inline]
    pub fn get_frame_with_timestamp(&mut self, time: &Timecode) -> Option<&mut FrameLeaf<T>> {
        self.get_frame_with_isize(time.frames())
    }

    #[inline]
    pub fn get_or_create_frame_with_isize(&mut self, frame: &isize) -> Option<&mut FrameLeaf<T>> {
        if self.get_frame_with_isize(frame).is_none() {
            return self.create_frame_with_isize(frame);
        }

        self.get_frame_with_isize(frame)
    }

    #[inline]
    pub fn get_or_create_frame_with_timestamp(
        &mut self,
        time: &Timecode,
    ) -> Option<&mut FrameLeaf<T>> {
        self.get_or_create_frame_with_isize(time.frames())
    }
}

#[derive(Debug)]
pub struct FrameLeaf<T: Tween> {
    nanos: BTreeMap<isize, Keyframe<T>>,
}

impl<T: Tween> Default for FrameLeaf<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Tween> FrameLeaf<T> {
    pub fn new() -> Self {
        Self {
            nanos: BTreeMap::new(),
        }
    }

    #[inline]
    pub fn add_keyframe_at_isize(
        &mut self,
        nanos: &isize,
        key: Keyframe<T>,
    ) -> Option<&mut Keyframe<T>> {
        self.nanos.insert(*nanos, key);
        self.get_keyframe_at_isize(nanos)
    }

    #[inline]
    pub fn add_keyframe_at_timestamp(
        &mut self,
        time: &Timecode,
        key: Keyframe<T>,
    ) -> Option<&mut Keyframe<T>> {
        self.add_keyframe_at_isize(time.nanoframes(), key)
    }

    #[inline]
    pub fn get_keyframe_at_isize(&mut self, frame: &isize) -> Option<&mut Keyframe<T>> {
        self.nanos.get_mut(frame)
    }

    #[inline]
    pub fn get_keyframe_at_timestamp(&mut self, time: &Timecode) -> Option<&mut Keyframe<T>> {
        self.get_keyframe_at_isize(time.nanoframes())
    }
}

#[derive(Debug, Default)]
pub struct Keyframe<T: Tween> {
    pub value: T,
}
