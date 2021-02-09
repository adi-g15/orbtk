// Internal font helper.
#[derive(Default, Clone, PartialEq, Debug)]
pub(crate) struct FontConfig {
    pub family: String,
    pub font_size: f64,
}

impl ToString for FontConfig {
    fn to_string(&self) -> String {
        format!("{}px {}", self.font_size, self.family)
    }
}
