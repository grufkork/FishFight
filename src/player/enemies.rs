use hecs::{World, Entity};
use macroquad::prelude::{Vec2, collections::storage, vec2};

use crate::{Transform, Drawable, PhysicsBody, Resources, AnimatedSpriteMetadata};

use super::{PlayerAttributes, PlayerController, Player, PlayerInventory, PlayerEventQueue, spawn_player, PlayerControllerKind, Ai};

pub struct Enemies;

impl Enemies{
    pub fn mook(world: &mut World, position: Vec2) -> Entity{
        let character = storage::get::<Resources>().player_characters.get("sharky").unwrap().clone();

        let enemy = spawn_player(
            world,
            2,
            position,
            PlayerControllerKind::Ai(Ai::create(2)),
            character
        );

        enemy
    }
}