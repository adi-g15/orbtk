use std::{any::Any, fmt};

use crate::RenderTarget;

pub trait RenderPipeline {
    /// Draws the ctx of the pipeline.
    fn draw(&self, image: &mut RenderTarget);
}

/// Used to implement a custom render pipeline.
pub trait PipelineTrait: RenderPipeline + Any + Send {
    /// Equality for two Pipeline objects.
    fn box_eq(&self, other: &dyn Any) -> bool;

    /// Converts self to an any reference.
    fn as_any(&self) -> &dyn Any;

    /// Clones self as box.
    fn clone_box(&self) -> Box<dyn PipelineTrait>;

    /// Draws the ctx of the pipeline.
    fn draw_pipeline(&self, image: &mut RenderTarget) {
        self.draw(image);
    }
}

impl PartialEq for Box<dyn PipelineTrait> {
    fn eq(&self, other: &Box<dyn PipelineTrait>) -> bool {
        self.box_eq(other.as_any())
    }
}

impl Clone for Box<dyn PipelineTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl fmt::Debug for Box<dyn PipelineTrait> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Box<dyn PipelineTrait>")
    }
}
