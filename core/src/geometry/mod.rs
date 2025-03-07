//! Geometry module.

mod point;
mod size;

pub use point::Point;
pub use size::Size;

use crate::primitives::Rectangle;

/// Adds the ability to get the bounding box of an item.
///
/// The exact definition of the bounding box depends on the item:
///
/// * Primitives ([`Rectangle`], [`Circle`], ...)
///
///    For unstyled [primitives] the bounding box is defined as the smallest rectangle that surrounds the entire primitive.
/// * Styled primitives and other [`Drawable`]s ([`Image`], [`Text`], ...)
///
///    The bounding box of a drawable is defined as the smallest rectangle that contains all drawn pixels.
///    While all builtin [`Drawable`]s in embedded-graphics provide an implementation of this trait, this might
///    not be true for third party drawables.
///
///    Note that a styled primitive can have a different bounding box than the underlying unstyled primitive;
///    depending on the stroke width and alignment the bounding box of the styled primitive may be larger.
/// * [`DrawTarget`]s (displays, simulator, ...)
///
///    The bounding box of a draw target is defined as the area that should be used for drawing operations.
///    For most display drivers the top left corner of the bounding box will be at the origin but other draw targets
///    can have different positions of the top left corner.
///
/// The bounding box will be returned as a [`Rectangle`]. The methods provided by [`Rectangle`] make
/// it easy to implement additional functions like hit testing (by using [`contains`]) or drawing a focus
/// rectangle around a drawable (by converting the rectangle into a [`Styled`]).
///
/// # Implementation notes
///
/// `Dimensions` should be implemented for `Drawable`s if the bounding box is known before [`Drawable::draw`] is
/// executed. The implementation must return a rectangle that contains all drawn pixels.
/// [`MockDisplay::affected_area`] can be a used in unit tests to make sure a drawable returns a bounding box with
/// the correct dimensions.
///
/// [`DrawTarget`]s  (display drivers, etc) are required to implement `Dimensions`. The
/// implementation must return a rectangle representing the drawing area. For display
/// drivers it is recommended to implement [`OriginDimensions`] instead of implementing `Dimensions` directly,
/// if the top left corner of the display area is at the origin `(0, 0)`.
///
/// The bounding box of [`ImageDrawable`]s must always start at the origin, therefore [`OriginDimensions`] must be implemented instead of this trait.
///
/// [`Drawable`]: super::Drawable
/// [`Drawable::draw`]: super::Drawable::draw
/// [`DrawTarget`]: super::draw_target::DrawTarget
/// [`ImageDrawable`]: super::image::ImageDrawable
/// [`Rectangle`]: super::primitives::rectangle::Rectangle
/// [`points`]: super::primitives::PointsIter
/// [`MockDisplay::affected_area`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/mock_display/struct.MockDisplay.html#method.affected_area
/// [`contains`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/primitives/trait.ContainsPoint.html#tymethod.contains
/// [primitives]: https://docs.rs/embedded-graphics/latest/embedded_graphics/primitives/index.html
/// [`Circle`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/primitives/circle/struct.Circle.html
/// [`Image`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/image/struct.Image.html
/// [`Text`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/fonts/struct.Text.html
/// [`Styled`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/style/styled/struct.Styled.html
pub trait Dimensions {
    /// Returns the bounding box.
    fn bounding_box(&self) -> Rectangle;
}

impl<D> Dimensions for &D
where
    D: Dimensions,
{
    fn bounding_box(&self) -> Rectangle {
        (**self).bounding_box()
    }
}

impl<D> Dimensions for &mut D
where
    D: Dimensions,
{
    fn bounding_box(&self) -> Rectangle {
        (**self).bounding_box()
    }
}

/// Dimensions with `top_left` of the bounding box at `(0, 0)`.
///
/// A blanket implementation of `Dimensions` is provided for all types that implement this trait.
/// See the [`Dimensions`] trait documentation for more information about bounding boxes.
///
/// # Implementation notes
///
/// This trait should be implemented instead of [`Dimensions`] if the top left corner of the bounding box
/// will always be at the origin, which will be the case for most display drivers. Some types, like [`ImageDrawable`],
/// require a bounding box that starts at the origin and can only be used if [`OriginDimensions`] is implemented.
///
/// [`ImageDrawable`]: super::image::ImageDrawable
pub trait OriginDimensions: Dimensions {
    /// Returns the size of the bounding box.
    fn size(&self) -> Size;
}

impl<D> OriginDimensions for &D
where
    D: OriginDimensions,
{
    fn size(&self) -> Size {
        (**self).size()
    }
}

impl<D> OriginDimensions for &mut D
where
    D: OriginDimensions,
{
    fn size(&self) -> Size {
        (**self).size()
    }
}

// impl<T> Dimensions for T
// where
//     T: OriginDimensions,
// {
//     fn bounding_box(&self) -> Rectangle {
//         Rectangle::new(Point::zero(), self.size())
//     }
// }

/// Anchor point.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum AnchorPoint {
    /// Top left.
    TopLeft,
    /// Top center.
    TopCenter,
    /// Top right.
    TopRight,
    /// Center left.
    CenterLeft,
    /// Center.
    Center,
    /// Center right.
    CenterRight,
    /// Bottom left.
    BottomLeft,
    /// Bottom center.
    BottomCenter,
    /// Bottom right.
    BottomRight,
}
