// Copyright 2024 the Interpoli Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use alloc::{string::String, vec::Vec};
use core::ops::Range;
use kurbo::{PathEl, Shape as _};

use hashbrown::HashMap;

use crate::{Brush, Repeater, Stroke, Transform, Value, animated};

/// Model of a Lottie file.
#[derive(Clone, Default, Debug)]
pub struct Composition {
    /// Frames in which the animation is active.
    pub frames: Range<f64>,
    /// Frames per second.
    pub frame_rate: f64,
    /// Width of the animation.
    pub width: usize,
    /// Height of the animation.
    pub height: usize,
    /// Precomposed layers that may be instanced.
    pub assets: HashMap<String, Vec<Layer>>,
    /// Collection of layers.
    pub layers: Vec<Layer>,
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

#[derive(Clone, Debug)]
pub struct Draw {
    /// Parameters for a stroked draw operation.
    pub stroke: Option<Stroke>,
    /// Brush for the draw operation.
    pub brush: Brush,
    /// Opacity of the draw operation.
    pub opacity: Value<f64>,
}

/// Elements of a shape layer.
#[derive(Clone, Debug)]
pub enum Shape {
    /// Group of shapes with an optional transform.
    Group(Vec<Shape>, Option<GroupTransform>),
    /// Geometry element.
    Geometry(Geometry),
    /// Fill or stroke element.
    Draw(Draw),
    /// Repeater element.
    Repeater(Repeater),
}

/// Transform and opacity for a shape group.
#[derive(Clone, Debug)]
pub struct GroupTransform {
    pub transform: Transform,
    pub opacity: Value<f64>,
}

/// Layer in an animation.
#[derive(Clone, Debug, Default)]
pub struct Layer {
    /// Name of the layer.
    pub name: String,
    /// Index of the transform parent layer.
    pub parent: Option<usize>,
    /// Transform for the entire layer.
    pub transform: Transform,
    /// Opacity for the entire layer.
    pub opacity: Value<f64>,
    /// Width of the layer.
    pub width: f64,
    /// Height of the layer.
    pub height: f64,
    /// Blend mode for the layer.
    pub blend_mode: Option<peniko::BlendMode>,
    /// Range of frames in which the layer is active.
    pub frames: Range<f64>,
    /// Frame time stretch factor.
    pub stretch: f64,
    /// Starting frame for the layer (only applied to instances).
    pub start_frame: f64,
    /// List of masks applied to the content.
    pub masks: Vec<Mask>,
    /// True if the layer is used as a mask.
    pub is_mask: bool,
    /// Mask blend mode and layer.
    pub mask_layer: Option<(peniko::BlendMode, usize)>,
    /// Content of the layer.
    pub content: Content,
}

/// Matte layer mode.
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum Matte {
    #[default]
    Normal,
    // TODO: Use these
    // Alpha,
    // InvertAlpha,
    // Luma,
    // InvertLuma,
}

/// Mask for a layer.
#[derive(Clone, Debug)]
pub struct Mask {
    /// Blend mode for the mask.
    pub mode: peniko::BlendMode,
    /// Geometry that defines the shape of the mask.
    pub geometry: Geometry,
    /// Opacity of the mask.
    pub opacity: Value<f64>,
}

/// Content of a layer.
#[derive(Clone, Default, Debug)]
pub enum Content {
    /// Empty layer.
    #[default]
    None,
    /// Asset instance with the specified name and time remapping.
    Instance {
        name: String,
        time_remap: Option<Value<f64>>,
    },
    /// Collection of shapes.
    Shape(Vec<Shape>),
}
