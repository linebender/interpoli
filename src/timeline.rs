// Copyright 2024 the Interpoli Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::{Easing, Tween};
use alloc::{
    collections::btree_map::BTreeMap,
    fmt::Debug,
    format,
    string::{String, ToString},
    vec::Vec,
};
use anymap::hashbrown::AnyMap;
use core::time::Duration;
use hashbrown::HashMap;

#[derive(Copy, Debug, Clone)]
pub enum Framerate {
    Timestamp,
    Fixed(f64),
    Interpolated(f64),
}

impl Framerate {
    pub fn as_string(&self) -> String {
        match self {
            Framerate::Timestamp => 0.0_f64.to_string(),
            Framerate::Fixed(f) | Framerate::Interpolated(f) => f.to_string(),
        }
    }

    pub fn as_f64(&self) -> f64 {
        match self {
            Framerate::Timestamp => 0.0_f64,
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
#[macro_export]
macro_rules! tcode_hmsf {
    ($h:tt:$m:tt:$s:tt:$f:tt) => {
        Timecode::new($h, $m, $s, $f)
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! tcode_hmsf_framerate {
    ($h:tt:$m:tt:$s:tt:$f:tt, $fr:expr) => {
        Timecode::new_with_framerate($h, $m, $s, $f, 0, $fr)
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! tcode_hms {
    ($h:tt:$m:tt:$s:tt) => {
        Timecode::new($h, $m, $s, 0)
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! tcode_full {
    ($h:tt:$m:tt:$s:tt:$f:tt:$nf:tt, $fr:expr) => {
        Timecode::new_with_framerate($h, $m, $s, $f, $nf, $fr)
    };
}

impl Default for Timecode {
    fn default() -> Self {
        Timecode::new_with_framerate(0, 0, 0, 0, 0, Framerate::Timestamp)
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

    #[inline]
    pub fn framerate(&self) -> &Framerate {
        &self.framerate
    }

    #[inline]
    pub fn set_framerate(&mut self, fr: Framerate) {
        self.framerate = fr;
    }

    pub fn correct_with_framerate(&mut self, fr: Framerate) -> &Self {
        self.framerate = fr;
        self.correct_overflow();
        self.correct_underflow();
        self
    }

    fn correct_overflow(&mut self) {
        let framerate = self.framerate.as_f64();

        while self.nanoframes > 999_999_999 {
            self.frames += 1;
            self.nanoframes -= 1_000_000_000;
        }

        if framerate != 0.0 {
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

        while self.nanoframes < 0 {
            self.frames -= 1;
            self.nanoframes += 1_000_000_000;
        }

        if framerate != 0.0 {
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
        let nanos = t.as_nanoseconds_with_framerate(&self.framerate, false);
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
        let nanos = t.as_nanoseconds_with_framerate(&self.framerate, false);
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
        self.as_nanoseconds_with_framerate(&self.framerate, false)
    }

    pub fn as_nanoseconds_with_framerate(&self, fr: &Framerate, for_tweening: bool) -> isize {
        let mut nanos: isize = 0;
        let framerate = if fr.as_f64() != 0.0 {
            fr.as_f64()
        } else {
            // If it's a timestamp, cancel the division.
            1.0
        };

        if matches!(fr, Framerate::Interpolated(_f)) || !for_tweening {
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

        let t = self.as_nanoseconds_with_framerate(&self.framerate, true);
        let a = begin.as_nanoseconds_with_framerate(&self.framerate, true);
        let b = end.as_nanoseconds_with_framerate(&self.framerate, true);

        let a_f64 = a as f64;
        let b_f64 = b as f64 - a_f64;
        let t_f64 = t as f64 - a_f64;

        let lerp = 0.0 + (b_f64 - 0.0) * (t_f64 / b_f64);

        lerp / b_f64
    }
}

#[derive(Debug)]
pub struct StaticTimeline<T: Tween> {
    time: Timecode,
    sequences: HashMap<usize, Sequence<T>>,
    children: Vec<StaticTimeline<T>>,
    max_sequences: usize,
    sequence_name_map: HashMap<String, usize>,
}

impl<T: Tween> StaticTimeline<T> {
    pub fn new(fr: Framerate) -> Self {
        Self {
            time: tcode_hmsf_framerate!(00:00:00:00, fr),
            sequences: HashMap::new(),
            children: Vec::new(),
            max_sequences: 0,
            sequence_name_map: HashMap::new(),
        }
    }

    #[inline]
    pub fn framerate(&self) -> &Framerate {
        self.time.framerate()
    }

    /// # Panics
    ///
    /// TODO!
    pub fn new_sequence(&mut self, name: &str) -> Option<&mut Sequence<T>> {
        self.max_sequences += 1;

        self.sequences
            .insert(self.max_sequences, Sequence::<T>::new());
        self.sequence_name_map
            .insert(name.to_string(), self.max_sequences);

        self.sequences.get_mut(&self.max_sequences)
    }

    #[inline]
    pub fn get_sequence_pointer(&self, name: &str) -> Option<&usize> {
        self.sequence_name_map.get(name)
    }

    /// # Panics
    ///
    /// TODO!
    #[inline]
    pub fn get_sequence_with_pointer(&mut self, pointer: usize) -> Option<&mut Sequence<T>> {
        self.sequences.get_mut(&pointer)
    }

    /// # Panics
    ///
    /// TODO!
    pub fn get_sequence_with_name(&mut self, name: &str) -> Option<&mut Sequence<T>> {
        let ptr = self.get_sequence_pointer(name).unwrap();

        self.get_sequence_with_pointer(*ptr)
    }

    pub fn add_child(&mut self, child: StaticTimeline<T>) {
        self.children.push(child);
    }

    pub fn get_child_mut(&mut self, id: usize) -> Option<&mut StaticTimeline<T>> {
        self.children.get_mut(id)
    }

    pub fn children(&self) -> &Vec<StaticTimeline<T>> {
        &self.children
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

    /// # Panics
    ///
    /// TODO!
    #[inline]
    pub fn tween_by_name(&mut self, sequence_name: &str) -> T {
        let time = self.time.clone();
        let sequence = self.get_sequence_with_name(sequence_name).unwrap();
        sequence.tween(&time)
    }

    /// # Panics
    ///
    /// TODO!
    #[inline]
    pub fn tween_by_pointer(&mut self, sequence_ptr: usize) -> T {
        let time = self.time.clone();
        let sequence = self.get_sequence_with_pointer(sequence_ptr).unwrap();
        sequence.tween(&time)
    }
}

#[derive(Debug)]
pub struct Timeline {
    time: Timecode,
    sequences: AnyMap,
    children: Vec<Timeline>,
    max_sequences: usize,
    sequence_name_map: HashMap<String, usize>,
}

impl Timeline {
    pub fn new(fr: Framerate) -> Self {
        Self {
            time: tcode_hmsf_framerate!(00:00:00:00, fr),
            sequences: AnyMap::new(),
            children: Vec::new(),
            max_sequences: 0,
            sequence_name_map: HashMap::new(),
        }
    }

    #[inline]
    pub fn framerate(&self) -> &Framerate {
        self.time.framerate()
    }

    /// # Panics
    ///
    /// TODO!
    pub fn new_sequence<T: Tween + 'static>(&mut self, name: &str) -> Option<&mut Sequence<T>> {
        self.max_sequences += 1;

        if self
            .sequences
            .get::<HashMap<usize, Sequence<T>>>()
            .is_none()
        {
            self.sequences.insert(HashMap::<usize, Sequence<T>>::new());
        }

        let seq_list = self
            .sequences
            .get_mut::<HashMap<usize, Sequence<T>>>()
            .unwrap();

        seq_list.insert(self.max_sequences, Sequence::<T>::new());

        self.sequence_name_map
            .insert(name.to_string(), self.max_sequences);

        seq_list.get_mut(&self.max_sequences)
    }

    #[inline]
    pub fn get_sequence_pointer(&self, name: &str) -> Option<&usize> {
        self.sequence_name_map.get(name)
    }

    /// # Panics
    ///
    /// TODO!
    pub fn get_sequence_with_pointer<T: Tween + 'static>(
        &mut self,
        pointer: usize,
    ) -> Option<&mut Sequence<T>> {
        let seq_list = self
            .sequences
            .get_mut::<HashMap<usize, Sequence<T>>>()
            .unwrap();

        seq_list.get_mut(&pointer)
    }

    /// # Panics
    ///
    /// TODO!
    pub fn get_sequence_with_name<T: Tween + 'static>(
        &mut self,
        name: &str,
    ) -> Option<&mut Sequence<T>> {
        let ptr = self.get_sequence_pointer(name).unwrap();

        self.get_sequence_with_pointer(*ptr)
    }

    pub fn add_child(&mut self, child: Timeline) {
        self.children.push(child);
    }

    pub fn get_child_mut(&mut self, id: usize) -> Option<&mut Timeline> {
        self.children.get_mut(id)
    }

    pub fn children(&self) -> &Vec<Timeline> {
        &self.children
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

    /// # Panics
    ///
    /// TODO!
    #[inline]
    pub fn tween_by_name<T: Tween + 'static>(&mut self, sequence_name: &str) -> T {
        let time = self.time.clone();
        let sequence = self.get_sequence_with_name(sequence_name).unwrap();
        sequence.tween(&time)
    }

    /// # Panics
    ///
    /// TODO!
    #[inline]
    pub fn tween_by_pointer<T: Tween + 'static>(&mut self, sequence_ptr: usize) -> T {
        let time = self.time.clone();
        let sequence = self.get_sequence_with_pointer(sequence_ptr).unwrap();
        sequence.tween(&time)
    }
}

#[derive(Debug)]
pub struct Sequence<T: Tween> {
    engine: AnimationEngine<T>,
    tree: BTreeMap<isize, SecondLeaf<T>>,
    last_time: Timecode,
}

impl<T: Tween> Default for Sequence<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Tween> Sequence<T> {
    pub fn new() -> Self {
        Self {
            engine: AnimationEngine::default(),
            tree: BTreeMap::new(),
            last_time: tcode_hmsf!(00:00:00:00),
        }
    }

    /// # Panics
    ///
    /// TODO!
    pub fn tween(&mut self, time: &Timecode) -> T {
        // TODO: Make it so it returns 'T::default' instead of panicking.
        if !self.engine.is_running() && !self.engine.is_sequence_ended() {
            let current_keyframe_binding =
                self.get_keyframes_between(&self.last_time, time, time.framerate());
            let current_keyframe = current_keyframe_binding.last().unwrap();

            let last_keyframe_wrapped =
                self.find_first_keyframe_after_timestamp(&current_keyframe.0, time.framerate());

            match last_keyframe_wrapped {
                Some(last_keyframe) => {
                    self.engine.set_new_animation(
                        current_keyframe.0.clone(),
                        last_keyframe.0.clone(),
                        current_keyframe.1.clone(),
                        last_keyframe.1.clone(),
                    );
                }
                None => {
                    self.engine.set_new_end(current_keyframe.1.clone());
                }
            }

            self.last_time.set_by_timestamp(time.clone());
            self.last_time.set_framerate(*time.framerate());
        }

        self.engine.tween(time)
    }

    // Create and get second

    #[inline]
    pub fn create_second_with_isize(&mut self, second: &isize) -> Option<&mut SecondLeaf<T>> {
        self.tree.insert(*second, SecondLeaf::<T>::new());
        self.get_second_with_isize(second)
    }

    #[inline]
    pub fn create_second_with_timestamp(&mut self, time: &Timecode) -> Option<&mut SecondLeaf<T>> {
        let hours_to_sec = time.hours() * 3600;
        let minutes_to_sec = time.minutes() * 60;
        self.create_second_with_isize(&(hours_to_sec + minutes_to_sec + time.seconds()))
    }

    #[inline]
    pub fn get_second_with_isize(&mut self, second: &isize) -> Option<&mut SecondLeaf<T>> {
        self.tree.get_mut(second)
    }

    #[inline]
    pub fn get_second_with_timestamp(&mut self, time: &Timecode) -> Option<&mut SecondLeaf<T>> {
        let hours_to_sec = time.hours() * 3600;
        let minutes_to_sec = time.minutes() * 60;
        self.get_second_with_isize(&(hours_to_sec + minutes_to_sec + time.seconds()))
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
        let hours_to_sec = time.hours() * 3600;
        let minutes_to_sec = time.minutes() * 60;
        self.get_or_create_second_with_isize(&(hours_to_sec + minutes_to_sec + time.seconds()))
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
        let second: &mut SecondLeaf<T> = self.get_or_create_second_with_timestamp(time).unwrap();
        let frame: &mut FrameLeaf<T> = second.get_or_create_frame_with_timestamp(time).unwrap();

        frame.add_keyframe_at_timestamp(time, key)
    }

    /// # Panics
    ///
    /// TODO!
    pub fn get_keyframe_at_timestamp(&mut self, time: &Timecode) -> Option<&mut Keyframe<T>> {
        // TODO: Make it so it returns 'None' instead of panicking.
        let second: &mut SecondLeaf<T> = self.get_second_with_timestamp(time).unwrap();
        let frame: &mut FrameLeaf<T> = second.get_frame_with_timestamp(time).unwrap();

        frame.get_keyframe_at_timestamp(time)
    }

    /// # Panics
    ///
    /// TODO!
    pub fn add_keyframes_at_timestamp(&mut self, keyframes: Vec<(Keyframe<T>, &Timecode)>) {
        for k in keyframes {
            // TODO: Make it so it returns 'None' instead of panicking.
            self.add_keyframe_at_timestamp(k.0, k.1);
        }
    }

    /// # Panics
    ///
    /// TODO!
    #[inline]
    pub fn get_keyframes_between(
        &self,
        begin: &Timecode,
        end: &Timecode,
        fr: &Framerate,
    ) -> Vec<(Timecode, Keyframe<T>)> {
        let mut final_vec: Vec<(Timecode, Keyframe<T>)> = Vec::new();

        let begin_secs = (begin.hours() * 3600) + (begin.minutes() * 60) + begin.seconds();
        let end_secs = (end.hours() * 3600) + (end.minutes() * 60) + end.seconds() + 1;

        for i in begin_secs..end_secs {
            let sec_leaf_search: Option<&SecondLeaf<T>> = self.tree.get(&i);

            if sec_leaf_search.is_none() {
                continue;
            }

            // TODO: Make it so it returns 'None' instead of panicking.
            let sec_leaf: &SecondLeaf<T> = sec_leaf_search.unwrap();

            sec_leaf.get_keyframes_between(begin, end, i, fr, &mut final_vec);
        }

        final_vec
    }

    /// # Panics
    ///
    /// TODO!
    #[inline]
    pub fn find_first_keyframe_after_timestamp(
        &self,
        timestamp: &Timecode,
        fr: &Framerate,
    ) -> Option<(Timecode, Keyframe<T>)> {
        let begin_secs =
            (timestamp.hours() * 3600) + (timestamp.minutes() * 60) + timestamp.seconds();
        let end_secs = *self.tree.iter().next_back().unwrap().0 + 1;

        for i in begin_secs..end_secs {
            let sec_leaf_search: Option<&SecondLeaf<T>> = self.tree.get(&i);

            if sec_leaf_search.is_none() {
                continue;
            }

            // TODO: Make it so it returns 'None' instead of panicking.
            let sec_leaf: &SecondLeaf<T> = sec_leaf_search.unwrap();

            let keyframe = sec_leaf.find_first_keyframe_after_timestamp(timestamp, i, fr);

            if keyframe.is_none() {
                continue;
            }

            return keyframe;
        }

        None
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
    pub fn get_keyframes_between(
        &self,
        begin: &Timecode,
        end: &Timecode,
        current_second: isize,
        fr: &Framerate,
        final_vec: &mut Vec<(Timecode, Keyframe<T>)>,
    ) {
        let begin_frames: isize = if current_second == *begin.seconds() {
            *begin.frames()
        } else {
            0
        };

        let end_frames: isize = if current_second == *end.seconds() {
            *end.frames()
        } else {
            fr.as_f64() as isize
        };

        for fra_leaf in &self.frames {
            if *fra_leaf.0 < begin_frames - 1 {
                continue;
            }

            if *fra_leaf.0 > end_frames {
                break;
            }

            fra_leaf.1.get_keyframes_between(
                begin,
                end,
                current_second,
                *fra_leaf.0,
                fr,
                final_vec,
            );
        }
    }

    #[inline]
    pub fn find_first_keyframe_after_timestamp(
        &self,
        timestamp: &Timecode,
        current_second: isize,
        fr: &Framerate,
    ) -> Option<(Timecode, Keyframe<T>)> {
        let begin_frames: isize = if current_second == *timestamp.seconds() {
            *timestamp.frames()
        } else {
            0
        };

        for fra_leaf in &self.frames {
            if *fra_leaf.0 < begin_frames {
                continue;
            }

            let keyframe = fra_leaf.1.find_first_keyframe_after_timestamp(
                timestamp,
                current_second,
                *fra_leaf.0,
                fr,
            );

            if keyframe.is_none() {
                continue;
            }

            return keyframe;
        }

        None
    }

    #[inline]
    pub fn get_or_create_frame_with_timestamp(
        &mut self,
        time: &Timecode,
    ) -> Option<&mut FrameLeaf<T>> {
        self.get_or_create_frame_with_isize(time.frames())
    }
}

#[derive(Debug, Clone)]
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

    #[inline]
    pub fn get_keyframes_between(
        &self,
        begin: &Timecode,
        end: &Timecode,
        current_second: isize,
        current_frame: isize,
        fr: &Framerate,
        final_vec: &mut Vec<(Timecode, Keyframe<T>)>,
    ) {
        let begin_nanos: isize =
            if current_second == *begin.seconds() && current_frame == *begin.frames() {
                *begin.nanoframes()
            } else {
                0
            };

        let end_nanos: isize = if current_second == *end.seconds() && current_frame == *end.frames()
        {
            *end.nanoframes()
        } else {
            1_000_000_000
        };

        for nano_leaf in &self.nanos {
            if *nano_leaf.0 < begin_nanos - 1 {
                continue;
            }

            if *nano_leaf.0 > end_nanos {
                break;
            }

            let nanoframes = *nano_leaf.0;

            final_vec.push((
                tcode_full!(00:00:current_second:current_frame:nanoframes, *fr),
                nano_leaf.1.clone(),
            ));
        }
    }

    #[inline]
    pub fn find_first_keyframe_after_timestamp(
        &self,
        timestamp: &Timecode,
        current_second: isize,
        current_frame: isize,
        fr: &Framerate,
    ) -> Option<(Timecode, Keyframe<T>)> {
        let begin_nanos: isize = if current_frame == *timestamp.frames() {
            *timestamp.nanoframes()
        } else {
            0
        };

        for nano_leaf in &self.nanos {
            if *nano_leaf.0 < begin_nanos {
                continue;
            }

            let nanoframes = *nano_leaf.0;

            let time = tcode_full!(00:00:current_second:current_frame:nanoframes, *fr);

            if !time.is_equals_to_hmsf(timestamp) {
                return Some((time, nano_leaf.1.clone()));
            }
        }

        None
    }
}

#[derive(Debug, Default)]
pub struct AnimationEngine<T: Tween> {
    t_begin: Timecode,
    t_end: Timecode,
    k_begin: Keyframe<T>,
    k_end: Keyframe<T>,
    status: AnimationEngineStatus,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum AnimationEngineStatus {
    #[default]
    Empty,
    Running,
    Ended,
    SequenceEnded,
}

impl<T: Tween> AnimationEngine<T> {
    pub fn tween(&mut self, current_time: &Timecode) -> T {
        if self.status == AnimationEngineStatus::SequenceEnded {
            return self.k_end.value.clone();
        }

        let time = current_time.get_lerp_time_between(&self.t_begin, &self.t_end);

        if time >= 1.0 {
            self.status = AnimationEngineStatus::Ended;
        }

        self.k_begin
            .value
            .tween(&self.k_end.value, time, &Easing::LERP)
    }

    pub fn set_new_animation(
        &mut self,
        begin: Timecode,
        end: Timecode,
        k_begin: Keyframe<T>,
        k_end: Keyframe<T>,
    ) {
        self.t_begin = begin;
        self.t_end = end;
        self.k_begin = k_begin;
        self.k_end = k_end;

        self.status = AnimationEngineStatus::Running;
    }

    pub fn set_new_end(&mut self, k: Keyframe<T>) {
        self.k_end = k;
        self.status = AnimationEngineStatus::SequenceEnded;
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.status, AnimationEngineStatus::Empty)
    }

    pub fn is_running(&self) -> bool {
        matches!(self.status, AnimationEngineStatus::Running)
    }

    pub fn has_ended(&self) -> bool {
        matches!(self.status, AnimationEngineStatus::Ended)
    }

    pub fn is_sequence_ended(&self) -> bool {
        matches!(self.status, AnimationEngineStatus::SequenceEnded)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Keyframe<T: Tween> {
    pub value: T,
}
