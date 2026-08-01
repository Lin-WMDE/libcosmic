// Copyright 2022 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

//! Select preferred fonts.

pub use iced::Font;
use iced_core::font::{Family, Weight};
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

#[inline]
pub fn default() -> Font {
    Font::from(crate::config::interface_font())
}

/// Starts scanning the installed fonts on a background thread.
///
/// Call it as the first statement of `main`, before localization and before the
/// configuration is read. The scan takes tens of milliseconds and nothing else at start-up
/// depends on it, but the first piece of text does - so every millisecond it starts late is
/// a millisecond added to the time before a window appears. Safe to call more than once.
#[inline]
pub fn prewarm() {
    iced::advanced::graphics::text::prewarm_font_system();
}

/// The weight [`semibold`] actually asks for, per family.
///
/// Resolved once per family and kept: the answer only changes when fonts are installed or
/// removed, which does not happen while an application runs.
static SEMIBOLD_WEIGHT: LazyLock<RwLock<HashMap<&'static str, Weight>>> =
    LazyLock::new(RwLock::default);

/// Weight 600 if the family has a face at 600, and 700 if it does not.
///
/// cosmic-text does not remember a failed match. A family with no face at the requested
/// weight makes it walk the whole font database, fail, and walk it again on the very next
/// request. Noto Sans - the interface font - carries 400 and 700 and nothing between
/// (Medium and Black are separate families), so every heading on screen paid for a walk:
/// one start-up of the file manager produced 42 of them.
///
/// Asking for 700 asks for the face the fallback settles on anyway, by weight distance, so
/// this changes what is drawn on screen in no way at all.
fn semibold_weight(font: &Font) -> Weight {
    let Family::Name(family) = font.family else {
        return Weight::Semibold;
    };

    if let Some(weight) = SEMIBOLD_WEIGHT
        .read()
        .ok()
        .and_then(|resolved| resolved.get(family).copied())
    {
        return weight;
    }

    // try_read, never read. This runs from `view` and from layout, and the text pipeline
    // holds this very lock while it shapes; blocking here would be waiting for a frame that
    // is waiting for us. Not looking costs nothing - the question is asked again next time.
    let Ok(font_system) = iced::advanced::graphics::text::font_system().try_read() else {
        return Weight::Semibold;
    };
    let has_semibold = font_system.db().faces().any(|face| {
        face.weight == iced::advanced::graphics::text::cosmic_text::fontdb::Weight::SEMIBOLD
            && face
                .families
                .iter()
                .any(|(name, _)| name.as_str() == family)
    });
    drop(font_system);

    let weight = if has_semibold {
        Weight::Semibold
    } else {
        Weight::Bold
    };
    if let Ok(mut resolved) = SEMIBOLD_WEIGHT.write() {
        resolved.insert(family, weight);
    }
    weight
}

#[inline]
pub fn light() -> Font {
    Font {
        weight: Weight::Light,
        ..default()
    }
}

#[inline]
pub fn semibold() -> Font {
    let font = default();
    Font {
        weight: semibold_weight(&font),
        ..font
    }
}

#[inline]
pub fn bold() -> Font {
    Font {
        weight: Weight::Bold,
        ..default()
    }
}

#[inline]
pub fn mono() -> Font {
    Font::from(crate::config::monospace_font())
}
