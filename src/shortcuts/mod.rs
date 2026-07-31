// SPDX-License-Identifier: GPL-3.0-only

//! Key bindings an application stores in its own configuration.
//!
//! This is the in-application half of WMDE shortcuts: bindings the focused window
//! handles itself, as opposed to the desktop-wide ones the compositor executes from
//! `fun.wmde.Settings.Shortcuts`. Both an application and the settings app read and
//! write the same stored shape through this module, so neither has to know how the
//! other spells a binding.
//!
//! An application supplies its own action type. [`Shortcuts`] maps a [`Binding`] to
//! one of those actions; [`ShortcutsConfig`] layers the user's map over the shipped
//! defaults, where an action that reports [`ShortcutAction::is_disable`] removes the
//! inherited binding instead of running anything.
//!
//! Licensed GPL-3.0-only, as the code it was lifted from: `src/localize.rs` already
//! puts this crate under the GPL when linked.

use crate::iced::core::keyboard::key::Named;
use crate::iced::keyboard::{Key, Modifiers};
use crate::widget::menu::key_bind::{KeyBind, Modifier};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// An action an application can bind a key combination to.
pub trait ShortcutAction: Clone + Eq {
    /// Whether this action removes an inherited binding rather than running anything.
    ///
    /// A custom map entry carrying such an action takes the default binding away; it
    /// is how a user drops a shipped shortcut without replacing it.
    fn is_disable(&self) -> bool;
}

/// A modifier as it is written in the configuration.
///
/// Stored by name rather than as a bitmask so that a configuration file stays
/// readable and diffable.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ModifierName {
    Ctrl,
    Shift,
    Alt,
    Super,
}

impl ModifierName {
    #[must_use]
    pub const fn to_modifier(self) -> Modifier {
        match self {
            Self::Ctrl => Modifier::Ctrl,
            Self::Shift => Modifier::Shift,
            Self::Alt => Modifier::Alt,
            Self::Super => Modifier::Super,
        }
    }
}

/// A key combination as it is written in the configuration.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Binding {
    pub modifiers: Vec<ModifierName>,
    pub key: String,
}

impl Binding {
    /// Builds a binding from a modifier list and a key name.
    pub fn new(modifiers: impl Into<Vec<ModifierName>>, key: impl Into<String>) -> Self {
        Self {
            modifiers: modifiers.into(),
            key: key.into(),
        }
    }

    /// Converts to the widget-level binding used for matching and display.
    ///
    /// Returns `None` for a key name that does not resolve, which is what an
    /// unreadable configuration entry looks like.
    #[must_use]
    pub fn to_key_bind(&self) -> Option<KeyBind> {
        let key = key_from_string(&self.key)?;
        // Emitted in a fixed order so that the same combination always displays the
        // same way, whatever order the configuration listed the modifiers in.
        let modifiers = [
            ModifierName::Ctrl,
            ModifierName::Shift,
            ModifierName::Alt,
            ModifierName::Super,
        ]
        .into_iter()
        .filter(|modifier| self.modifiers.contains(modifier))
        .map(ModifierName::to_modifier)
        .collect();

        Some(KeyBind { modifiers, key })
    }
}

/// A map of key combinations to actions.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Shortcuts<A>(pub BTreeMap<Binding, A>);

impl<A> Default for Shortcuts<A> {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl<A> Shortcuts<A> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Binding, &A)> {
        self.0.iter()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<A> FromIterator<(Binding, A)> for Shortcuts<A> {
    fn from_iter<I: IntoIterator<Item = (Binding, A)>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

/// Where a resolved binding came from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BindingSource {
    Default,
    Custom,
}

/// A binding together with the map it was found in.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedBinding {
    pub binding: Binding,
    pub source: BindingSource,
}

/// The user's bindings layered over the ones the application ships.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ShortcutsConfig<A> {
    defaults: Shortcuts<A>,
    pub custom: Shortcuts<A>,
}

impl<A: ShortcutAction> ShortcutsConfig<A> {
    pub fn new(defaults: Shortcuts<A>, custom: Shortcuts<A>) -> Self {
        Self { defaults, custom }
    }

    /// The bindings the application ships, before the user's map is applied.
    #[must_use]
    pub fn defaults(&self) -> &Shortcuts<A> {
        &self.defaults
    }

    /// Resolves both maps into the bindings an application matches against.
    ///
    /// `to_action` turns a stored action into whatever the caller dispatches on, and
    /// returns `None` for an action the running build cannot perform - an action
    /// behind a disabled feature, for instance.
    pub fn key_binds<T, F>(&self, mut to_action: F) -> HashMap<KeyBind, T>
    where
        F: FnMut(&A) -> Option<T>,
    {
        let mut binds = HashMap::new();
        self.insert(&self.defaults, &mut binds, false, &mut to_action);
        self.insert(&self.custom, &mut binds, true, &mut to_action);
        binds
    }

    fn insert<T, F>(
        &self,
        shortcuts: &Shortcuts<A>,
        binds: &mut HashMap<KeyBind, T>,
        allow_disable: bool,
        to_action: &mut F,
    ) where
        F: FnMut(&A) -> Option<T>,
    {
        for (binding, action) in &shortcuts.0 {
            let Some(key_bind) = binding.to_key_bind() else {
                tracing::warn!(?binding, "invalid key binding");
                continue;
            };
            if allow_disable && action.is_disable() {
                binds.remove(&key_bind);
                continue;
            }
            let Some(action) = to_action(action) else {
                tracing::warn!(?binding, "unsupported shortcut action");
                continue;
            };
            binds.insert(key_bind, action);
        }
    }

    /// Every binding that currently triggers `action`, and whether the user changed it.
    ///
    /// The flag is what a settings page needs to decide whether to offer a reset: it
    /// is set both when a custom binding was added and when a default was taken away.
    pub fn bindings_for_action(&self, action: &A) -> (Vec<ResolvedBinding>, bool) {
        let mut bindings = Vec::new();
        let mut changed = false;

        for (binding, default_action) in &self.defaults.0 {
            if default_action != action {
                continue;
            }

            match self.custom.0.get(binding) {
                Some(custom_action) if custom_action.is_disable() => changed = true,
                Some(custom_action) => {
                    if custom_action == action {
                        bindings.push(ResolvedBinding {
                            binding: binding.clone(),
                            source: BindingSource::Custom,
                        });
                        changed = true;
                    }
                }
                None => bindings.push(ResolvedBinding {
                    binding: binding.clone(),
                    source: BindingSource::Default,
                }),
            }
        }

        for (binding, custom_action) in &self.custom.0 {
            if custom_action == action
                && !bindings.iter().any(|resolved| resolved.binding == *binding)
            {
                bindings.push(ResolvedBinding {
                    binding: binding.clone(),
                    source: BindingSource::Custom,
                });
                changed = true;
            }
        }

        (bindings, changed)
    }

    /// The action a combination triggers, or `None` if it triggers nothing.
    pub fn action_for_binding(&self, binding: &Binding) -> Option<&A> {
        if let Some(action) = self.custom.0.get(binding) {
            return if action.is_disable() {
                None
            } else {
                Some(action)
            };
        }

        self.defaults.0.get(binding)
    }

    /// Drops every custom entry that concerns `action`, restoring its defaults.
    ///
    /// Both directions have to go: the entries that bind the action elsewhere, and
    /// the entries that overrode - or disabled - the combinations it shipped with.
    pub fn reset_action(&mut self, reset_action: &A) {
        self.custom.0.retain(|binding, action| {
            if action == reset_action {
                return false;
            }
            !self
                .defaults
                .0
                .get(binding)
                .is_some_and(|default_action| default_action == reset_action)
        });
    }
}

/// How a binding is spelled in the interface.
#[must_use]
pub fn binding_display(binding: &Binding) -> String {
    binding
        .to_key_bind()
        .map_or_else(|| binding.key.clone(), |key_bind| key_bind.to_string())
}

/// Builds a binding from a key press, for a settings page that captures one.
///
/// Returns `None` while only modifiers are held, so that capture keeps waiting for
/// the key that completes the combination.
#[must_use]
pub fn binding_from_key(modifiers: Modifiers, key: &Key) -> Option<Binding> {
    if is_modifier_only_key(key) {
        return None;
    }
    let key = key_to_string(key)?;
    let mut binding_modifiers = Vec::new();
    if modifiers.control() {
        binding_modifiers.push(ModifierName::Ctrl);
    }
    if modifiers.shift() {
        binding_modifiers.push(ModifierName::Shift);
    }
    if modifiers.alt() {
        binding_modifiers.push(ModifierName::Alt);
    }
    if modifiers.logo() {
        binding_modifiers.push(ModifierName::Super);
    }
    Some(Binding {
        modifiers: binding_modifiers,
        key,
    })
}

/// Resolves a stored key name.
#[must_use]
pub fn key_from_string(value: &str) -> Option<Key> {
    match value {
        "Backspace" => Some(Key::Named(Named::Backspace)),
        "Enter" => Some(Key::Named(Named::Enter)),
        "Escape" => Some(Key::Named(Named::Escape)),
        "Insert" => Some(Key::Named(Named::Insert)),
        "Delete" => Some(Key::Named(Named::Delete)),
        "Tab" => Some(Key::Named(Named::Tab)),
        "F1" => Some(Key::Named(Named::F1)),
        "F2" => Some(Key::Named(Named::F2)),
        "F3" => Some(Key::Named(Named::F3)),
        "F4" => Some(Key::Named(Named::F4)),
        "F5" => Some(Key::Named(Named::F5)),
        "F6" => Some(Key::Named(Named::F6)),
        "F7" => Some(Key::Named(Named::F7)),
        "F8" => Some(Key::Named(Named::F8)),
        "F9" => Some(Key::Named(Named::F9)),
        "F10" => Some(Key::Named(Named::F10)),
        "F11" => Some(Key::Named(Named::F11)),
        "F12" => Some(Key::Named(Named::F12)),
        "Home" => Some(Key::Named(Named::Home)),
        "End" => Some(Key::Named(Named::End)),
        "ArrowLeft" | "Left" => Some(Key::Named(Named::ArrowLeft)),
        "ArrowRight" | "Right" => Some(Key::Named(Named::ArrowRight)),
        "ArrowUp" | "Up" => Some(Key::Named(Named::ArrowUp)),
        "ArrowDown" | "Down" => Some(Key::Named(Named::ArrowDown)),
        "PageUp" => Some(Key::Named(Named::PageUp)),
        "PageDown" => Some(Key::Named(Named::PageDown)),
        "Space" | "space" => Some(Key::Character(" ".into())),
        _ if !value.is_empty() => Some(Key::Character(value.into())),
        _ => None,
    }
}

/// Writes a captured key back as a stored key name.
#[must_use]
pub fn key_to_string(key: &Key) -> Option<String> {
    match key {
        Key::Character(c) => {
            if c == " " {
                Some("Space".to_string())
            } else if c.len() == 1 && c.chars().all(|ch| ch.is_ascii_alphabetic()) {
                Some(c.to_uppercase())
            } else {
                Some(c.to_string())
            }
        }
        Key::Named(named) => Some(format!("{named:?}")),
        _ => None,
    }
}

/// Whether a key only ever appears alongside another one.
#[must_use]
pub fn is_modifier_only_key(key: &Key) -> bool {
    matches!(
        key,
        Key::Named(
            Named::Alt
                | Named::AltGraph
                | Named::CapsLock
                | Named::Control
                | Named::Fn
                | Named::FnLock
                | Named::NumLock
                | Named::ScrollLock
                | Named::Shift
                | Named::Symbol
                | Named::SymbolLock
                | Named::Meta
                | Named::Hyper
                | Named::Super
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
    enum Action {
        Disable,
        New,
        Close,
    }

    impl ShortcutAction for Action {
        fn is_disable(&self) -> bool {
            matches!(self, Self::Disable)
        }
    }

    fn config(custom: &[(Binding, Action)]) -> ShortcutsConfig<Action> {
        let defaults = Shortcuts::from_iter([
            (Binding::new([ModifierName::Ctrl], "T"), Action::New),
            (Binding::new([ModifierName::Ctrl], "W"), Action::Close),
        ]);
        ShortcutsConfig::new(defaults, Shortcuts::from_iter(custom.to_vec()))
    }

    #[test]
    fn custom_binding_wins_over_the_default() {
        let binding = Binding::new([ModifierName::Ctrl], "T");
        let config = config(&[(binding.clone(), Action::Close)]);

        assert_eq!(config.action_for_binding(&binding), Some(&Action::Close));
    }

    #[test]
    fn disable_takes_the_binding_away() {
        let binding = Binding::new([ModifierName::Ctrl], "T");
        let config = config(&[(binding.clone(), Action::Disable)]);

        assert_eq!(config.action_for_binding(&binding), None);
        assert!(
            !config.key_binds(|a| Some(*a)).contains_key(
                &binding
                    .to_key_bind()
                    .expect("Ctrl+T has to resolve to a key bind")
            )
        );
    }

    #[test]
    fn a_disabled_default_still_counts_as_changed() {
        let config = config(&[(Binding::new([ModifierName::Ctrl], "T"), Action::Disable)]);

        let (bindings, changed) = config.bindings_for_action(&Action::New);
        assert!(bindings.is_empty());
        assert!(
            changed,
            "the reset button has to appear for a disabled default"
        );
    }

    #[test]
    fn reset_drops_both_the_override_and_the_new_binding() {
        let mut config = config(&[
            // Overrides the default combination of New.
            (Binding::new([ModifierName::Ctrl], "T"), Action::Disable),
            // Binds New somewhere else.
            (Binding::new([ModifierName::Alt], "N"), Action::New),
            // Nothing to do with New.
            (Binding::new([ModifierName::Alt], "W"), Action::Close),
        ]);

        config.reset_action(&Action::New);

        assert_eq!(config.custom.len(), 1);
        let (bindings, changed) = config.bindings_for_action(&Action::New);
        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].source, BindingSource::Default);
        assert!(!changed);
    }

    #[test]
    fn modifier_order_does_not_change_the_display() {
        let one = Binding::new([ModifierName::Shift, ModifierName::Ctrl], "T");
        let other = Binding::new([ModifierName::Ctrl, ModifierName::Shift], "T");

        assert_eq!(binding_display(&one), binding_display(&other));
    }

    #[test]
    fn a_key_alone_is_not_a_binding_until_it_is_not_a_modifier() {
        assert_eq!(
            binding_from_key(Modifiers::CTRL, &Key::Named(Named::Control)),
            None
        );
        assert_eq!(
            binding_from_key(Modifiers::CTRL, &Key::Character("t".into())),
            Some(Binding::new([ModifierName::Ctrl], "T"))
        );
    }
}
