use bevy::prelude::*;

pub const FONT_COLOR: Color = Color::BLACK;

pub enum RenderLayer {
    Landscape,
    Decoration,
    RoutingGrid,
    Airport,
    Airplane,
    Ui,
}

impl RenderLayer {
    pub fn z(&self) -> f32 {
        match self {
            RenderLayer::Landscape => 0.0,
            RenderLayer::Decoration => 1.0,
            RenderLayer::RoutingGrid => 2.0,
            RenderLayer::Airport => 3.0,
            RenderLayer::Airplane => 4.0,
            RenderLayer::Ui => 5.0,
        }
    }
}
