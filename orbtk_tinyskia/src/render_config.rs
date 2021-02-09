use crate::*;
use orbtk_base::utils::*;

/// Defines the current configuration of the render ctx.
#[derive(Debug, Clone)]
pub(crate) struct RenderConfig {
    pub fill_style: Brush,
    pub stroke_style: Brush,
    pub line_width: f64,
    pub font_config: FontConfig,
    pub alpha: f32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        RenderConfig {
            fill_style: Brush::default(),
            stroke_style: Brush::default(),
            line_width: 1.,
            font_config: FontConfig::default(),
            alpha: 1.,
        }
    }
}
