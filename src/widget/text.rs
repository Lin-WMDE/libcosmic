use crate::Renderer;
pub use iced::widget::Text;
use iced_core::text::LineHeight;
use std::borrow::Cow;

/// Creates a new [`Text`] widget with the provided content.
///
/// [`Text`]: widget::Text
pub fn text<'a>(text: impl Into<Cow<'a, str>> + 'a) -> Text<'a, crate::Theme, Renderer> {
    Text::new(text.into()).font(crate::font::default())
}

/// Available presets for text typography
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Typography {
    Body,
    Caption,
    CaptionHeading,
    Heading,
    Monotext,
    Title1,
    Title2,
    Title3,
    Title4,
}

/// [`Text`] widget with the Title 1 typography preset.
pub fn title1<'a>(text: impl Into<Cow<'a, str>> + 'a) -> Text<'a, crate::Theme, Renderer> {
    #[inline(never)]
    fn inner(text: Cow<str>) -> Text<crate::Theme, Renderer> {
        Text::new(text)
            .size(35.0)
            .line_height(LineHeight::Absolute(52.0.into()))
            .font(crate::font::semibold())
    }

    inner(text.into())
}

/// [`Text`] widget with the Title 2 typography preset.
pub fn title2<'a>(text: impl Into<Cow<'a, str>> + 'a) -> Text<'a, crate::Theme, Renderer> {
    #[inline(never)]
    fn inner(text: Cow<str>) -> Text<crate::Theme, Renderer> {
        Text::new(text)
            .size(29.0)
            .line_height(LineHeight::Absolute(43.0.into()))
            .font(crate::font::semibold())
    }

    inner(text.into())
}

/// [`Text`] widget with the Title 3 typography preset.
pub fn title3<'a>(text: impl Into<Cow<'a, str>> + 'a) -> Text<'a, crate::Theme, Renderer> {
    #[inline(never)]
    fn inner(text: Cow<str>) -> Text<crate::Theme, Renderer> {
        Text::new(text)
            .size(24.0)
            .line_height(LineHeight::Absolute(36.0.into()))
            .font(crate::font::bold())
    }

    inner(text.into())
}

/// [`Text`] widget with the Title 4 typography preset.
pub fn title4<'a>(text: impl Into<Cow<'a, str>> + 'a) -> Text<'a, crate::Theme, Renderer> {
    #[inline(never)]
    fn inner(text: Cow<str>) -> Text<crate::Theme, Renderer> {
        Text::new(text)
            .size(20.0)
            .line_height(LineHeight::Absolute(30.0.into()))
            .font(crate::font::bold())
    }

    inner(text.into())
}

/// [`Text`] widget with the Heading typography preset.
///
/// WMDE: 12px/17, like every other run of interface text in the stack - Windows
/// sizes its chrome at Segoe UI 9pt (12px at 96 DPI) and separates a heading from
/// body text by weight, not by size. The toolkit's 14px stays only in the Title
/// presets, which are page headings rather than chrome.
pub fn heading<'a>(text: impl Into<Cow<'a, str>> + 'a) -> Text<'a, crate::Theme, Renderer> {
    #[inline(never)]
    fn inner(text: Cow<str>) -> Text<crate::Theme, Renderer> {
        Text::new(text)
            .size(12.0)
            .line_height(LineHeight::Absolute(iced::Pixels(17.0)))
            .font(crate::font::bold())
    }

    inner(text.into())
}

/// [`Text`] widget with the Caption Heading typography preset.
pub fn caption_heading<'a>(text: impl Into<Cow<'a, str>> + 'a) -> Text<'a, crate::Theme, Renderer> {
    #[inline(never)]
    fn inner(text: Cow<str>) -> Text<crate::Theme, Renderer> {
        Text::new(text)
            .size(12.0)
            .line_height(LineHeight::Absolute(iced::Pixels(17.0)))
            .font(crate::font::semibold())
    }

    inner(text.into())
}

/// [`Text`] widget with the Body typography preset.
///
/// WMDE: 12px/17, see [`heading`]. Body and Caption now share their metrics, which
/// is how Windows reads too: secondary text there is the same size, dimmed.
pub fn body<'a>(text: impl Into<Cow<'a, str>> + 'a) -> Text<'a, crate::Theme, Renderer> {
    #[inline(never)]
    fn inner(text: Cow<str>) -> Text<crate::Theme, Renderer> {
        Text::new(text)
            .size(12.0)
            .line_height(LineHeight::Absolute(17.0.into()))
            .font(crate::font::default())
    }

    inner(text.into())
}

/// [`Text`] widget with the Caption typography preset.
pub fn caption<'a>(text: impl Into<Cow<'a, str>> + 'a) -> Text<'a, crate::Theme, Renderer> {
    #[inline(never)]
    fn inner(text: Cow<str>) -> Text<crate::Theme, Renderer> {
        Text::new(text)
            .size(12.0)
            .line_height(LineHeight::Absolute(17.0.into()))
            .font(crate::font::default())
    }

    inner(text.into())
}

/// [`Text`] widget with the Monotext typography preset.
///
/// WMDE: 12px/17, see [`heading`]. This is the monospace run inside ordinary
/// interface text (paths, hashes); the terminal sets its own font and size.
pub fn monotext<'a>(text: impl Into<Cow<'a, str>> + 'a) -> Text<'a, crate::Theme, Renderer> {
    #[inline(never)]
    fn inner(text: Cow<str>) -> Text<crate::Theme, Renderer> {
        Text::new(text)
            .size(12.0)
            .line_height(LineHeight::Absolute(17.0.into()))
            .font(crate::font::mono())
    }

    inner(text.into())
}
