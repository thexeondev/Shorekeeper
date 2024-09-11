use std::{cell::RefMut, sync::LazyLock};

use super::{ecs::world::World, player::Player};

mod movement;
use movement::MovementSystem;

macro_rules! enabled_systems {
    ($($sys:ident;)*) => {
        static SYSTEMS: LazyLock<Box<[Box<dyn System>]>> = LazyLock::new(|| {
            vec![
                $(Box::new($sys::new()) as Box<dyn System>,)*
            ].into_boxed_slice()
        });

        pub fn tick_systems(world: &mut World, players: &mut [::std::cell::RefMut<Player>]) {
            SYSTEMS.iter().for_each(|system| system.tick(world, players));
        }
    };
}

pub trait System: Send + Sync + 'static {
    fn tick(&self, world: &mut World, players: &mut [RefMut<Player>]);
}

enabled_systems! {
    MovementSystem;
}
