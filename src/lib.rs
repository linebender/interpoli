// Copyright 2024 the Interpoli Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Interpoli

// LINEBENDER LINT SET - lib.rs - v3
// See https://linebender.org/wiki/canonical-lints/
// These lints shouldn't apply to examples or tests.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
// These lints shouldn't apply to examples.
#![warn(clippy::print_stdout, clippy::print_stderr)]
// Targeting e.g. 32-bit means structs containing usize can give false positives for 64-bit.
#![cfg_attr(target_pointer_width = "64", warn(clippy::trivially_copy_pass_by_ref))]
// END LINEBENDER LINT SET
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
// The following lints are part of the Linebender standard set,
// but resolving them has been deferred for now.
// Feel free to send a PR that solves one or more of these.
#![allow(
    missing_docs,
    single_use_lifetimes,
    elided_lifetimes_in_paths,
    clippy::use_self,
    clippy::cast_possible_truncation,
    clippy::exhaustive_enums,
    reason = "Deferred"
)]

extern crate alloc;

use kurbo::Affine;

mod composition;
mod spline;
mod value;

#[cfg(feature = "vello")]
mod render;

pub mod animated;
pub mod fixed;

pub use composition::{
    Composition, Content, Draw, Geometry, GroupTransform, Layer, Mask, Matte, Shape,
};
pub use value::{Animated, Easing, EasingHandle, Time, Tween, Value, ValueRef};

#[cfg(feature = "vello")]
pub use render::Renderer;

macro_rules! simple_value {
    ($name:ident) => {
        #[derive(Clone, Debug)]
        #[allow(
            clippy::allow_attributes,
            reason = "This doesn't happen for all invocations."
        )]
        #[allow(clippy::large_enum_variant, reason = "It is how it is.")]
        pub enum $name {
            Fixed(fixed::$name),
            Animated(animated::$name),
        }

        impl $name {
            pub fn is_fixed(&self) -> bool {
                matches!(self, Self::Fixed(_))
            }
            pub fn evaluate(&self, frame: f64) -> ValueRef<'_, fixed::$name> {
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

    pub fn evaluate(&self, alpha: f64, frame: f64) -> ValueRef<'_, fixed::Brush> {
        match self {
            Self::Fixed(value) => {
                if alpha == 1.0 {
                    ValueRef::Borrowed(value)
                } else {
                    ValueRef::Owned(value.clone().multiply_alpha(alpha as f32))
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

#[cfg(test)]
mod tests {
    // CI will fail unless cargo nextest can execute at least one test per workspace.
    // Delete this dummy test once we have an actual real test.
    #[test]
    fn dummy_test_until_we_have_a_real_test() {}
}
