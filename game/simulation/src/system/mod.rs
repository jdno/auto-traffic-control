use hecs::World;

pub use self::despawn_airplane::*;
pub use self::detect_collision::*;
pub use self::generate_flight_plan::*;
pub use self::move_airplane::*;
pub use self::spawn_airplane::*;

mod despawn_airplane;
mod detect_collision;
mod generate_flight_plan;
mod move_airplane;
mod spawn_airplane;

pub trait System {
    fn update(&mut self, world: &mut World, delta: f32);
}
