use hecs::World;

pub use self::spawn_airplane::*;

mod spawn_airplane;

pub trait System {
    fn update(&mut self, world: &mut World, delta: f32);
}
