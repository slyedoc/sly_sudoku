use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};
use bevy_tweening::{lens::*, *};

pub struct LensPlugin;

impl Plugin for LensPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(component_animator_system::<Camera2d>.label(AnimationSystem::AnimationUpdate));
        app.add_system(component_animator_system::<BackgroundColor>.label(AnimationSystem::AnimationUpdate));
    }
}

/// A lens to manipulate the clear color of a camera2d
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Camera2dClearColorLens {
    /// Start color.
    pub start: Color,
    /// End color.
    pub end: Color,    
}

impl Lens<Camera2d> for Camera2dClearColorLens {
    fn lerp(&mut self, target: &mut Camera2d, ratio: f32) {
        let start: Vec4 = self.start.into();
        let end: Vec4 = self.end.into();
        let value = start.lerp(end, ratio);
        target.clear_color = ClearColorConfig::Custom(value.into());        
    }
}


/// A lens to manipulate the clear color of a camera2d
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BackgroundColorLens {
    /// Start color.
    pub start: Color,
    /// End color.
    pub end: Color,    
}

impl Lens<BackgroundColor> for BackgroundColorLens {
    fn lerp(&mut self, target: &mut BackgroundColor, ratio: f32) {
        let start: Vec4 = self.start.into();
        let end: Vec4 = self.end.into();
        let value = start.lerp(end, ratio);
        target.0 = value.into();        
    }
}