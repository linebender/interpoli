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

<<<<<<< HEAD
<<<<<<< HEAD
#[cfg(feature = "vello")]
mod render;

pub mod animated;
pub mod fixed;

pub use composition::{
    Composition, Content, Draw, Geometry, GroupTransform, Layer, Mask, Matte, Shape,
};
=======
=======

>>>>>>> 6fb3d22 (Apply changes from main)
#[macro_use]
pub mod timeline;
pub mod animated;
pub mod fixed;

pub use timeline::{Frame, Smpte, Timeline};
<<<<<<< HEAD
>>>>>>> dc11ff9 (Add SMPTE & Timeline)
=======

#[cfg(feature = "vello")]
mod render;

pub mod animated;
pub mod fixed;

pub use composition::{
    Composition, Content, Draw, Geometry, GroupTransform, Layer, Mask, Matte, Shape,
};

>>>>>>> 6fb3d22 (Apply changes from main)
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
<<<<<<< HEAD
=======

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
fn smpte_macro() {
    println!("smpte_macro: {:?}", smpte_hmsf!(1;23;45;01.0).as_string());
}

#[test]
fn smpte_macro_overflow() {
    println!("smpte_macro_overflow: {:?}", smpte_hmsf!(99;99;99;99.9).as_string());
}

#[test]
fn smpte_set_hms() {
    println!("smpte_set_hms: {:?}", smpte_hms!(98;76;54).hms_as_string());
}

#[test]
fn smpte_with_framerate() {
    println!("smpte_with_framerate: {:?}", smpte_hmsf_framerate!(00;01;02;56.0, Frame::Fixed(20.0)).as_string());
}

#[test]
fn smpte_full_24fps_second() {

    let mut time = smpte_hmsf_framerate!(00;00;00;00.0, Frame::Fixed(24.0));

    for i in 0..24 {
        time.next_frame();
    }

    println!("smpte_full_24f_second: {:?}", time.as_string());
}

#[test]
fn smpte_full_24fps_minute() {
    let mut time = smpte_hmsf_framerate!(00;00;00;00.0, Frame::Fixed(24.0));

    for i in 0..60 {
        time.next_second();
    }

    println!("smpte_full_24f_minute: {:?}", time.as_string());
}

#[test]
fn smpte_full_24fps_hour() {
    let mut time = smpte_hmsf_framerate!(00;00;00;00.0, Frame::Fixed(24.0));

    for i in 0..60 {
        time.next_minute();
    }

    println!("smpte_full_24f_hour: {:?}", time.as_string());
}
<<<<<<< HEAD
<<<<<<< HEAD
>>>>>>> dc11ff9 (Add SMPTE & Timeline)
=======
=======
>>>>>>> 39d8b23cd583bd4689a492efd969d0a60c143b79
>>>>>>> 6fb3d22 (Apply changes from main)
=======
>>>>>>> a005dff (Remove whatever this is)
