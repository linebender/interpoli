// Copyright 2024 the Interpoli Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![warn(unused_crate_dependencies)]

//! # Interpoli

extern crate alloc;

use kurbo::Affine;

mod composition;
mod spline;
mod value;

#[macro_use]
pub mod timeline;
pub mod animated;
pub mod fixed;

pub use timeline::{Framerate, Keyframe, Sequence, StaticTimeline, Timecode, Timeline};

#[cfg(feature = "vello")]
mod render;

pub use composition::{
    Composition, Content, Draw, Geometry, GroupTransform, Layer, Mask, Matte, Shape,
};

pub use value::{Animated, Easing, EasingHandle, Time, Tween, Value, ValueRef};

#[cfg(feature = "vello")]
pub use render::Renderer;

macro_rules! simple_value {
    ($name:ident) => {
        #[allow(clippy::large_enum_variant)]
        #[derive(Clone, Debug)]
        pub enum $name {
            Fixed(fixed::$name),
            Animated(animated::$name),
        }

        impl $name {
            pub fn is_fixed(&self) -> bool {
                matches!(self, Self::Fixed(_))
            }
            pub fn evaluate(&self, frame: f64) -> ValueRef<fixed::$name> {
                match self {
                    Self::Fixed(value) => ValueRef::Borrowed(value),
                    Self::Animated(value) => ValueRef::Owned(value.evaluate(frame)),
                }
            }
        }
    };
}

simple_value!(Transform);
simple_value!(Stroke);
simple_value!(Repeater);
simple_value!(ColorStops);

#[derive(Clone, Debug)]
pub enum Brush {
    Fixed(fixed::Brush),
    Animated(animated::Brush),
}

impl Brush {
    pub fn is_fixed(&self) -> bool {
        matches!(self, Self::Fixed(_))
    }

    pub fn evaluate(&self, alpha: f64, frame: f64) -> ValueRef<fixed::Brush> {
        match self {
            Self::Fixed(value) => {
                if alpha == 1.0 {
                    ValueRef::Borrowed(value)
                } else {
                    ValueRef::Owned(fixed::brush_with_alpha(value, alpha))
                }
            }
            Self::Animated(value) => ValueRef::Owned(value.evaluate(alpha, frame)),
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::Fixed(Affine::IDENTITY)
    }
}

// Syntax Tests.

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tcode_macro() {
    println!("tcode_macro: {:?}", tcode_hmsf!(1:23:45:01).as_string());
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tcode_set_hms() {
    println!("tcode_set_hms: {:?}", tcode_hms!(98:76:54).hms_as_string());
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tcode_with_framerate() {
    println!(
        "tcode_with_framerate: {:?}",
        tcode_hmsf_framerate!(00:01:02:56, Framerate::Fixed(20.0)).as_string()
    );
}

// Assert Tests.

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tcode_macro_overflow() {
    let time = tcode_hmsf!(99:99:99:99);

    println!(
        "tcode_macro_overflow: {:?}",
        tcode_hmsf!(99:99:99:99).as_string()
    );
    assert!(time.is_equals_to_hmsf(&tcode_hmsf!(100:40:39:99)));
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tcode_full_24fps_second() {
    let mut time = tcode_hmsf_framerate!(00:00:00:00, Framerate::Fixed(24.0));

    for _i in 0..24 {
        time.next_frame();
    }

    println!("tcode_full_24f_second: {:?}", time.as_string());
    assert!(time.is_equals_to_hmsf(&tcode_hmsf!(00:00:01:00)));
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tcode_full_24fps_minute() {
    let mut time = tcode_hmsf_framerate!(00:00:00:00, Framerate::Fixed(24.0));

    for _i in 0..60 {
        time.next_second();
    }

    println!("tcode_full_24f_minute: {:?}", time.as_string());
    assert!(time.is_equals_to_hmsf(&tcode_hmsf!(00:01:00:00)));
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tcode_full_24fps_hour() {
    let mut time = tcode_hmsf_framerate!(00:00:00:00, Framerate::Fixed(24.0));

    for _i in 0..60 {
        time.next_minute();
    }

    println!("tcode_full_24f_hour: {:?}", time.as_string());
    assert!(time.is_equals_to_hmsf(&tcode_hmsf!(01:00:00:00)));
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tcode_set_by_duration() {
    use core::time::Duration;

    let mut time = tcode_hmsf_framerate!(00:00:10:00, Framerate::Fixed(23.97));

    time.set_by_duration(Duration::from_secs(2));

    println!("tcode_set_by_duration: {:?}", time.as_string());
    assert!(time.is_equals_to_hmsf(&tcode_hmsf!(00:00:02:00)));
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tcode_set_by_timestamp() {
    let mut time = tcode_hmsf_framerate!(00:00:10:00, Framerate::Fixed(23.97));

    time.set_by_timestamp(tcode_hmsf!(00:05:00:00));

    println!("tcode_set_by_timestamp: {:?}", time.as_string());
    assert!(time.is_equals_to_hmsf(&tcode_hmsf!(00:05:00:00)));
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tcode_add_by_duration() {
    use core::time::Duration;

    let mut time = tcode_hmsf_framerate!(00:00:00:00, Framerate::Fixed(24.0));

    time.add_by_duration(Duration::from_millis(999));

    println!("tcode_add_by_duration: {:?}", time.as_string());
    assert!(time.is_equals_to_hmsf(&tcode_hmsf!(00:00:00:23)));
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tcode_add_by_timestamp() {
    let mut time = tcode_hmsf_framerate!(00:00:05:00, Framerate::Fixed(24.0));

    time.add_by_timestamp(tcode_hmsf!(00:00:05:00));

    println!("tcode_add_by_timestamp: {:?}", time.as_string());
    assert!(time.is_equals_to_hmsf(&tcode_hmsf!(00:00:10:00)));
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tcode_sub_by_duration() {
    use core::time::Duration;

    let mut time = tcode_hmsf_framerate!(01:00:00:00, Framerate::Fixed(24.0));

    time.sub_by_duration(Duration::from_secs(1800));

    println!("tcode_sub_by_duration: {:?}", time.as_string());
    assert!(time.is_equals_to_hmsf(&tcode_hmsf!(00:30:00:00)));
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tcode_sub_by_timestamp() {
    let mut time = tcode_hmsf_framerate!(00:01:00:00, Framerate::Fixed(24.0));

    time.sub_by_timestamp(tcode_hmsf!(00:00:20:00));

    println!("tcode_sub_by_timestamp: {:?}", time.as_string());
    assert!(time.is_equals_to_hmsf(&tcode_hmsf!(00:00:40:00)));
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tcode_ntsc_tv() {
    use core::time::Duration;

    let mut time = tcode_hmsf_framerate!(00:00:00:00, Framerate::Fixed(23.97));

    time.add_by_duration(Duration::from_millis(2000));

    println!("tcode_ntsc_tv: {:?}", time.as_string());
    assert!(time.is_equals_to_hmsf(&tcode_hmsf!(00:00:02:00)));
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tcode_high_fps() {
    use core::time::Duration;

    let mut time = tcode_hmsf_framerate!(00:00:00:00, Framerate::Fixed(1000.0));

    time.add_by_duration(Duration::from_millis(2000));

    println!("tcode_high_fps: {:?}", time.as_string());
    assert!(time.is_equals_to_hmsf(&tcode_hmsf!(00:00:02:00)));
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tcode_framerate_standard_that_doesnt_exist() {
    use core::time::Duration;

    let mut time = tcode_hmsf_framerate!(00:00:00:00, Framerate::Fixed(159.3947));

    time.add_by_duration(Duration::from_millis(2000));

    println!(
        "tcode_framerate_standard_that_doesnt_exist: {:?}",
        time.as_string()
    );
    assert!(time.is_equals_to_hmsf(&tcode_hmsf!(00:00:02:00)));
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tcode_as_nanoseconds() {
    use core::time::Duration;

    let time = tcode_hmsf_framerate!(00:00:09:00, Framerate::Fixed(30.0));
    let dur = Duration::from_secs(9);

    println!(
        "tcode_as_nanoseconds: {:?} ({:?}) = {:?}",
        time.as_nanoseconds(),
        time.as_string(),
        dur.as_nanos()
    );
    assert!(time.as_nanoseconds() == dur.as_nanos() as isize);
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tcode_get_lerp_time_between() {
    let time = tcode_hmsf_framerate!(00:00:00:12, Framerate::Fixed(24.0));

    let begin = tcode_hmsf!(00:00:00:00);
    let end = tcode_hmsf!(00:00:00:24);

    let result = time.get_lerp_time_between(&begin, &end);

    println!("tcode_get_lerp_time_between: {:?}", result);
    assert!(result == 0.5);
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tcode_lerp_fixed_vs_inter() {
    let time_fixed = tcode_full!(00:00:00:00:500_000_000, Framerate::Fixed(1.0));
    let time_inter = tcode_full!(00:00:00:00:500_000_000, Framerate::Interpolated(1.0));

    let begin = tcode_hmsf!(00:00:00:00);
    let end = tcode_hmsf!(00:00:00:01);

    let fixed_result = time_fixed.get_lerp_time_between(&begin, &end);
    let inter_result = time_inter.get_lerp_time_between(&begin, &end);

    println!(
        "tcode_lerp_fixed_vs_inter: fixed {:?} | interpolated {:?}",
        fixed_result, inter_result
    );
    assert!(fixed_result == 0.0 && inter_result == 0.5);
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tline_new() {
    let timeline = Timeline::new(Framerate::Fixed(24.0));

    assert!(timeline.time().is_equals_to_hmsf(&tcode_hmsf!(00:00:00:00)));
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tline_set_by_timestamp() {
    let mut timeline = Timeline::new(Framerate::Fixed(24.0));

    timeline.set_by_timestamp(tcode_hmsf!(00:00:05:00));

    assert!(timeline.time().is_equals_to_hmsf(&tcode_hmsf!(00:00:05:00)));
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tline_new_integer_sequences() {
    let mut timeline = Timeline::new(Framerate::Fixed(24.0));

    let sequence_one: &mut Sequence<f64> = timeline.new_sequence("sequence_one").unwrap();

    assert!(sequence_one
        .add_keyframe_at_timestamp(Keyframe { value: 3.0 }, &tcode_hmsf!(00:00:05:00))
        .is_some());

    let sequence_two: &mut Sequence<f32> = timeline.new_sequence("sequence_two").unwrap();

    assert!(sequence_two
        .add_keyframe_at_timestamp(Keyframe { value: 6.0 }, &tcode_hmsf!(00:00:10:00))
        .is_some());
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tline_new_kurbo_sequences() {
    use kurbo::Vec2;

    #[cfg(feature = "std")]
    use std::time::Instant;

    let mut timeline = Timeline::new(Framerate::Fixed(24.0));

    let sequence: &mut Sequence<Vec2> = timeline.new_sequence("sequence").unwrap();

    sequence.add_keyframes_at_timestamp(vec![
        (
            Keyframe {
                value: Vec2::new(0.0, 1.0),
            },
            &tcode_hmsf!(00:00:01:00),
        ),
        (
            Keyframe {
                value: Vec2::new(1.0, 1.0),
            },
            &tcode_hmsf!(00:00:02:00),
        ),
        (
            Keyframe {
                value: Vec2::new(1.0, 2.0),
            },
            &tcode_hmsf!(00:00:03:00),
        ),
    ]);

    #[cfg(feature = "std")]
    let instant = Instant::now();

    assert!(sequence
        .get_keyframe_at_timestamp(&tcode_hmsf!(00:00:02:00))
        .is_some());

    #[cfg(feature = "std")]
    println!("tline_new_kurbo_sequences: {:?}", instant.elapsed());
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn static_tline_new_kurbo_sequences() {
    use kurbo::Vec2;

    #[cfg(feature = "std")]
    use std::time::Instant;

    let mut timeline: StaticTimeline<Vec2> = StaticTimeline::new(Framerate::Fixed(24.0));

    let sequence = timeline.new_sequence("sequence").unwrap();

    sequence.add_keyframes_at_timestamp(vec![
        (
            Keyframe {
                value: Vec2::new(0.0, 1.0),
            },
            &tcode_hmsf!(00:00:01:00),
        ),
        (
            Keyframe {
                value: Vec2::new(1.0, 1.0),
            },
            &tcode_hmsf!(00:00:02:00),
        ),
        (
            Keyframe {
                value: Vec2::new(1.0, 2.0),
            },
            &tcode_hmsf!(00:00:03:00),
        ),
    ]);

    #[cfg(feature = "std")]
    let instant = Instant::now();

    assert!(sequence
        .get_keyframe_at_timestamp(&tcode_hmsf!(00:00:02:00))
        .is_some());

    #[cfg(feature = "std")]
    println!("tline_new_kurbo_sequences: {:?}", instant.elapsed());
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tline_nesting() {
    use kurbo::Vec2;

    let mut main = Timeline::new(Framerate::Fixed(24.0));

    let main_seq: &mut Sequence<f64> = main.new_sequence("main_seq").unwrap();
    main_seq.add_keyframe_at_timestamp(Keyframe { value: 0.0 }, &tcode_hmsf!(00:00:02:00));

    let mut child = Timeline::new(Framerate::Fixed(24.0));

    let child_seq: &mut Sequence<Vec2> = child.new_sequence("child_seq").unwrap();
    child_seq.add_keyframe_at_timestamp(
        Keyframe {
            value: Vec2::new(0.0, 1.0),
        },
        &tcode_hmsf!(00:00:02:00),
    );

    main.add_child(child);

    assert!(main.children().len() == 1);
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tline_stress_test() {
    #[cfg(feature = "std")]
    use std::time::Instant;

    let mut timeline = Timeline::new(Framerate::Fixed(24.0));
    let framerate = *timeline.framerate();

    let sequence_one: &mut Sequence<f64> = timeline.new_sequence("stress_test").unwrap();

    for i in 0..1_000_000 {
        sequence_one.add_keyframe_at_timestamp(
            Keyframe { value: i as f64 },
            &tcode_hmsf_framerate!(00:00:i:00, framerate),
        );
    }

    #[cfg(feature = "std")]
    let instant = Instant::now();

    let keyframe = sequence_one.get_keyframe_at_timestamp(&tcode_hmsf!(00:15:00:00));

    #[cfg(feature = "std")]
    println!("tline_stress_test: {:?}", instant.elapsed());

    assert!(keyframe.is_some());
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn static_tline_stress_test() {
    #[cfg(feature = "std")]
    use std::time::Instant;

    let mut timeline: StaticTimeline<f64> = StaticTimeline::new(Framerate::Fixed(24.0));
    let framerate = *timeline.framerate();

    let sequence_one = timeline.new_sequence("stress_test").unwrap();

    for i in 0..1_000_000 {
        sequence_one.add_keyframe_at_timestamp(
            Keyframe { value: i as f64 },
            &tcode_hmsf_framerate!(00:00:i:00, framerate),
        );
    }

    #[cfg(feature = "std")]
    let instant = Instant::now();

    let keyframe = sequence_one.get_keyframe_at_timestamp(&tcode_hmsf!(00:15:00:00));

    #[cfg(feature = "std")]
    println!("static_tline_stress_test: {:?}", instant.elapsed());

    assert!(keyframe.is_some());
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn sequence_add_keyframe_at_timestamp() {
    let mut seq = Sequence::<f64>::new();

    assert!(seq
        .add_keyframe_at_timestamp(Keyframe { value: 0.5 }, &tcode_hmsf!(00:00:05:00))
        .is_some());
    assert!(seq
        .add_keyframe_at_timestamp(Keyframe { value: 1.0 }, &tcode_hmsf!(00:00:10:00))
        .is_some());
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn sequence_get_keyframes_between() {
    let mut seq = Sequence::<f64>::new();

    assert!(seq
        .add_keyframe_at_timestamp(Keyframe { value: 0.5 }, &tcode_hmsf!(00:00:05:00))
        .is_some());
    assert!(seq
        .add_keyframe_at_timestamp(Keyframe { value: 1.0 }, &tcode_hmsf!(00:02:00:00))
        .is_some());

    let keyframes = seq.get_keyframes_between(
        &tcode_hmsf!(00:00:00:00),
        &tcode_hmsf!(00:05:00:00),
        &Framerate::Fixed(24.0),
    );

    assert!(keyframes.len() == 2);

    println!("sequence_get_keyframes_between: {:?}", keyframes);
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn sequence_find_first_keyframe_after_timestamp() {
    #[cfg(feature = "std")]
    use std::time::Instant;

    let mut seq = Sequence::<f64>::new();

    assert!(seq
        .add_keyframe_at_timestamp(Keyframe { value: 0.5 }, &tcode_hmsf!(00:00:05:00))
        .is_some());
    assert!(seq
        .add_keyframe_at_timestamp(Keyframe { value: 1.0 }, &tcode_hmsf!(00:00:06:00))
        .is_some());

    #[cfg(feature = "std")]
    let instant = Instant::now();

    let keyframe =
        seq.find_first_keyframe_after_timestamp(&tcode_hmsf!(00:00:05:00), &Framerate::Fixed(24.0));

    #[cfg(feature = "std")]
    println!("time: {:?}", instant.elapsed());

    assert!(keyframe.is_some());

    println!(
        "sequence_find_first_keyframe_after_timestamp: {:?}",
        keyframe
    );
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tline_basic_linear_animation() {
    let mut timeline = Timeline::new(Framerate::Fixed(60.0));

    let sequence: &mut Sequence<f64> = timeline.new_sequence("animation_sequence").unwrap();

    sequence.add_keyframe_at_timestamp(Keyframe { value: 0.0 }, &tcode_hmsf!(00:00:00:00));
    sequence.add_keyframe_at_timestamp(Keyframe { value: 1.0 }, &tcode_hmsf!(00:00:01:00));

    timeline.add_by_timestamp(tcode_hmsf!(00:00:00:30));

    let result: f64 = timeline.tween_by_name("animation_sequence");

    println!("tline_animation_test_one: {:?}", result);
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn tline_animation_fixed_vs_inter() {
    let mut timeline_one = Timeline::new(Framerate::Fixed(1.0));
    let mut timeline_two = Timeline::new(Framerate::Interpolated(1.0));

    let sequence_one: &mut Sequence<f64> = timeline_one.new_sequence("sequence").unwrap();
    let sequence_two: &mut Sequence<f64> = timeline_two.new_sequence("sequence").unwrap();

    sequence_one.add_keyframe_at_timestamp(Keyframe { value: 0.0 }, &tcode_hmsf!(00:00:00:00));
    sequence_one.add_keyframe_at_timestamp(Keyframe { value: 1.0 }, &tcode_hmsf!(00:00:00:01));

    println!("tline_animation_fixed_vs_inter (fixed):");
    println!("========================");

    for i in 0..10 {
        timeline_one.add_by_timestamp(tcode_full!(00:00:00:00:100_000_000, Framerate::Timestamp));
        println!(
            "(Nanoframe {:?}): {:?}",
            (i + 1) * 100_000_000,
            timeline_one.tween_by_name::<f64>("sequence")
        );
    }

    sequence_two.add_keyframe_at_timestamp(Keyframe { value: 0.0 }, &tcode_hmsf!(00:00:00:00));
    sequence_two.add_keyframe_at_timestamp(Keyframe { value: 1.0 }, &tcode_hmsf!(00:00:00:01));

    println!("========================");
    println!("tline_animation_fixed_vs_inter (inter):");
    println!("========================");

    for i in 0..10 {
        timeline_two.add_by_timestamp(tcode_full!(00:00:00:00:100_000_000, Framerate::Timestamp));
        println!(
            "(Nanoframe {:?}): {:?}",
            (i + 1) * 100_000_000,
            timeline_two.tween_by_name::<f64>("sequence"),
        );
    }

    println!("========================");
}
