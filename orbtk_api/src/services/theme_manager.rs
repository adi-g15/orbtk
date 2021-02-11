use std::collections::HashMap;

use crate::theming::Theme;

pub struct ThemeManager {
    selected_theme: Option<Theme>,
    themes: HashMap<String, Theme>,
}

impl ThemeManager {
    pub fn new() -> Self {
        ThemeManager {
            selected_theme: None,
            themes: HashMap::new(),
        }
    }
}
