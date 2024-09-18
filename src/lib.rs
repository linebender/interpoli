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

pub use timeline::{Framerate, Timecode, Timeline};

#[cfg(feature = "vello")]
mod render;

pub use composition::{
    Composition, Content, Draw, GroupTransform, Layer, Mask, Matte,
};

pub use value::{Animated, Easing, EasingHandle, Time, Tween, Value, ValueRef};

pub use kurbo::{PathEl, Shape};

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

#[derive(Clone, Debug)]
pub enum Geometry {
    Fixed(Vec<PathEl>),
    Rect(animated::Rect),
    Ellipse(animated::Ellipse),
    Spline(animated::Spline),
}

impl Geometry {
    pub fn evaluate(&self, frame: f64, path: &mut Vec<PathEl>) {
        match self {
            Self::Fixed(value) => {
                path.extend_from_slice(value);
            }
            Self::Rect(value) => {
                path.extend(value.evaluate(frame).path_elements(0.1));
            }
            Self::Ellipse(value) => {
                path.extend(value.evaluate(frame).path_elements(0.1));
            }
            Self::Spline(value) => {
                value.evaluate(frame, path);
            }
        }
    }
}

#[test]
fn tcode_macro() {
    println!("tcode_macro: {:?}", tcode_hmsf!(1;23;45;01.0).as_string());
}

#[test]
fn tcode_macro_overflow() {
    println!("tcode_macro_overflow: {:?}", tcode_hmsf!(99;99;99;99.9).as_string());
}

#[test]
fn tcode_set_hms() {
    println!("tcode_set_hms: {:?}", tcode_hms!(98;76;54).hms_as_string());
}

#[test]
fn tcode_with_framerate() {
    println!("tcode_with_framerate: {:?}", tcode_hmsf_framerate!(00;01;02;56.0, Framerate::Fixed(20.0)).as_string());
}

#[test]
fn tcode_full_24fps_second() {

    let mut time = tcode_hmsf_framerate!(00;00;00;00.0, Framerate::Fixed(24.0));

    for i in 0..24 {
        time.next_frame();
    }

    println!("tcode_full_24f_second: {:?}", time.as_string());
}

#[test]
fn tcode_full_24fps_minute() {
    let mut time = tcode_hmsf_framerate!(00;00;00;00.0, Framerate::Fixed(24.0));

    for i in 0..60 {
        time.next_second();
    }

    println!("tcode_full_24f_minute: {:?}", time.as_string());
}

#[test]
fn tcode_full_24fps_hour() {
    let mut time = tcode_hmsf_framerate!(00;00;00;00.0, Framerate::Fixed(24.0));

    for i in 0..60 {
        time.next_minute();
    }

    println!("tcode_full_24f_hour: {:?}", time.as_string());
}

#[test]
fn tcode_add_by_duration() {

    use std::time::Duration;

    let mut time = tcode_hmsf_framerate!(00;00;00;00.0, Framerate::Fixed(24.0));

    time.add_by_duration(Duration::from_millis(999));

    println!("tcode_add_by_duration: {:?}", time.as_string());
}

#[test]
fn tcode_sub_by_duration() {

    use std::time::Duration;

    let mut time = tcode_hmsf_framerate!(01;00;00;00.0, Framerate::Fixed(24.0));

    time.sub_by_duration(Duration::from_secs(1800));

    println!("tcode_sub_by_duration: {:?}", time.as_string());
}