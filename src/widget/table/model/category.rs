use std::borrow::Cow;

use crate::widget::Icon;

/// Implementation of std::fmt::Display allows user to customize the header
/// Ideally, this is implemented on an enum.
pub trait ItemCategory:
    Default + std::fmt::Display + Clone + Copy + PartialEq + Eq + std::hash::Hash
{
    /// Function that gets the width of the data
    fn width(&self) -> iced::Length;

    /// Which edge of the column the heading and the cells line up against.
    ///
    /// A number read against its neighbours has to line up on the right; a name reads on the
    /// left. Defaults to the left so that an existing table keeps the layout it had.
    fn align(&self) -> iced::alignment::Horizontal {
        iced::alignment::Horizontal::Left
    }
}

pub trait ItemInterface<Category: ItemCategory> {
    fn get_icon(&self, category: Category) -> Option<Icon>;
    fn get_text(&self, category: Category) -> Cow<'static, str>;

    fn compare(&self, other: &Self, category: Category) -> std::cmp::Ordering;
}
