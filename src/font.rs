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

/// Weight 600, or 700 for a static family that has no face at 600.
///
/// Noto Sans, the interface font, has no face at weight 600, and asking for one is not free.
/// Measured on the VM: one start-up of the file manager logged
/// `No default font match for Name("Noto Sans") at weight 600` **42 times**, each one a pass
/// over the font database. With this substitution it logs none, and the rendered window is
/// identical to the pixel.
///
/// The repetition is not simply a missing cache - `get_font_matches` in cosmic-text is
/// memoized on (family, stretch, style, weight) - but something clears or bypasses that
/// cache during start-up, and the 42 passes were measured, not deduced. Asking for 700 asks
/// for the face cosmic-text settles on anyway, by weight distance, so nothing drawn changes.
///
/// A **variable** family is left alone, and the check is built around that. cosmic-text
/// feeds the requested weight straight into the `wght` axis, so a variable family really
/// does draw 600 at 600 and 700 at 700 - substituting there would make headings visibly
/// heavier. fontdb has no flag for it and exposes such a family as a single face at its
/// default weight, so the family is treated as static only when it shows **two or more
/// distinct weights**. One weight means either a variable family or a family with nothing
/// to substitute; both are better left as they are.
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

    use iced::advanced::graphics::text::cosmic_text::fontdb;
    let mut weights: Vec<u16> = font_system
        .db()
        .faces()
        .filter(|face| face.families.iter().any(|(name, _)| name.as_str() == family))
        .map(|face| face.weight.0)
        .collect();
    drop(font_system);
    weights.sort_unstable();
    weights.dedup();

    let weight = if weights.len() >= 2 && !weights.contains(&fontdb::Weight::SEMIBOLD.0) {
        Weight::Bold
    } else {
        Weight::Semibold
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
