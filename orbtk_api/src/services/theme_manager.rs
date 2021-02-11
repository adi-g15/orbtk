use std::collections::HashMap;

use crate::{theming::Theme, widget_base::Context};

pub struct ThemeManager {
    selected_theme_key: Option<String>,
    themes: HashMap<String, Theme>,
}

impl ThemeManager {
    pub fn new() -> Self {
        ThemeManager {
            selected_theme_key: None,
            themes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: impl Into<String>, theme: Theme) {
        let key = key.into();
        self.themes.insert(key.clone(), theme);
        self.selected_theme_key = Some(key);
    }

    pub fn selected_theme(&self, key: impl Into<String>) -> Option<&Theme> {
        self.themes.get(&key.into())
    }

    pub fn len(&self) -> usize {
        self.themes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.themes.is_empty()
    }

    // todo update values of widgets before rendering
}
