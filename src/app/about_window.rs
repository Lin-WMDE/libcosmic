// Copyright 2025 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

//! A standard About window, built and owned by the framework.
//!
//! Applications open it with [`super::ApplicationExt::open_about`] rather than rendering
//! the about widget themselves. The window is dispatched before
//! [`super::Application::view_window`], so applications never see its id.

use crate::widget::about::About;
use crate::{Apply, Element, fl};
use iced::{Alignment, Length, Size, window};

/// Fixed outer size of the About window.
const WINDOW_SIZE: Size = Size::new(480.0, 600.0);

/// Widest the content may get, matching the context drawer's cap.
const CONTENT_MAX_WIDTH: f32 = 480.0;

/// Settings for the About window of an application.
#[cfg_attr(not(target_os = "linux"), allow(unused_variables))]
pub(crate) fn settings(application_id: &str) -> window::Settings {
    #[allow(unused_mut)]
    let mut settings = window::Settings {
        size: WINDOW_SIZE,
        // Equal minimum and maximum sizes make the compositor treat this as a dialog, so
        // it floats instead of joining the tiling layout.
        min_size: Some(WINDOW_SIZE),
        max_size: Some(WINDOW_SIZE),
        resizable: false,
        // Server side decorations: the compositor draws the same header bar widget.
        decorations: true,
        transparent: true,
        ..Default::default()
    };

    // Without this the window has no app id at all, so it gets no taskbar icon.
    #[cfg(target_os = "linux")]
    {
        settings.platform_specific.application_id = application_id.to_string();
    }

    settings
}

/// Title of the About window.
///
/// Must be applied before the window opens: the title is read once, when the window is
/// created, and there is no action to change it afterwards.
pub(crate) fn title(about: &About) -> String {
    about
        .get_name()
        .map_or_else(|| fl!("about"), |name| fl!("about-app", name = name))
}

/// View for the About window.
pub(crate) fn view<M: Clone + 'static>(about: &About) -> Element<'_, crate::Action<M>> {
    let cosmic_theme::Spacing {
        space_l, space_m, ..
    } = crate::theme::spacing();

    // The about widget is only the content column: the padding, the width cap, the
    // scrollbar and the background were all supplied by the context drawer before.
    crate::widget::about(about, |url| {
        crate::Action::Cosmic(super::Action::AboutUrl(url.to_string()))
    })
    .apply(crate::widget::container)
    .padding([space_m, space_l, space_l, space_l])
    .width(Length::Fill)
    .max_width(CONTENT_MAX_WIDTH)
    .apply(crate::widget::container)
    .width(Length::Fill)
    .align_x(Alignment::Center)
    .apply(crate::widget::scrollable)
    .height(Length::Fill)
    .apply(crate::widget::container)
    .width(Length::Fill)
    .height(Length::Fill)
    .class(crate::theme::Container::WindowBackground)
    .into()
}

/// Opens a link from the About window in the user's preferred application.
#[cfg(not(windows))]
pub(crate) fn open_url<M: Send + 'static>(url: String) -> super::Task<M> {
    // A license without a license url still renders a pressable row.
    if url.is_empty() {
        return iced::Task::none();
    }

    let mut command = std::process::Command::new("xdg-open");
    command.arg(&url);

    iced::Task::future(async move {
        if crate::process::spawn(command).await.is_none() {
            tracing::warn!(%url, "failed to open link from the about window");
        }
    })
    .discard()
}

#[cfg(windows)]
pub(crate) fn open_url<M: Send + 'static>(_url: String) -> super::Task<M> {
    iced::Task::none()
}
