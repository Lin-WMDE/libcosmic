// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

//! A collection of tabs for developing a tabbed interface.
//!
//! See the [`segmented_button`] module for more details.

use super::segmented_button::{
    self, HorizontalSegmentedButton, Model, SegmentedButton, Selectable, VerticalSegmentedButton,
};

/// A collection of tabs for developing a tabbed interface.
///
/// The data for the widget comes from a model supplied by the application.
///
/// For details on the model, see the [`segmented_button`] module for more details.
pub fn horizontal<SelectionMode: Default, Message: Clone + 'static>(
    model: &Model<SelectionMode>,
) -> HorizontalSegmentedButton<'_, SelectionMode, Message>
where
    Model<SelectionMode>: Selectable,
{
    let space_xs = crate::theme::spacing().space_xs;

    segmented_button::horizontal(model)
        .minimum_button_width(76)
        .maximum_button_width(250)
        .button_height(44)
        // WMDE: Windows tab metrics. Side padding is 12 rather than 16, and the label
        // of the active tab keeps the regular weight: Explorer marks the active tab by
        // the fill that merges into the strip below it, not by a heavier label. The
        // label size itself comes from the segmented button default, now 12px.
        .button_padding([space_xs, space_xs, space_xs, space_xs])
        .font_active(crate::font::default())
        .style(crate::theme::SegmentedButton::TabBar)
}

/// A collection of tabs for developing a tabbed interface.
///
/// The data for the widget comes from a model that is maintained the application.
/// For details on the model, see the [`segmented_button`] module for more details.
pub fn vertical<SelectionMode, Message: Clone + 'static>(
    model: &Model<SelectionMode>,
) -> VerticalSegmentedButton<'_, SelectionMode, Message>
where
    Model<SelectionMode>: Selectable,
    SelectionMode: Default,
{
    let space_xs = crate::theme::spacing().space_xs;

    SegmentedButton::new(model)
        .minimum_button_width(76)
        .maximum_button_width(250)
        .button_height(44)
        // WMDE: same tab metrics as the horizontal strip above.
        .button_padding([space_xs, space_xs, space_xs, space_xs])
        .font_active(crate::font::default())
        .style(crate::theme::SegmentedButton::TabBar)
}
