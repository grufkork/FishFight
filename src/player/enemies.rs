use hecs::{World, Entity};
use macroquad::prelude::Vec2;

struct Enemies;

impl Enemies{
    fn mook(world: &mut World, position: Vec2) -> Entity{
        let weapon_mount = character.weapon_mount;

        let offset = storage::get::<Resources>()
            .textures
            .get(&character.sprite.texture_id)
            .map(|t| {
                let frame_size = t.frame_size();
                character.sprite.offset
                    - vec2(frame_size.x / 2.0, frame_size.y - character.collider_size.y)
            })
            .unwrap();

        let animations = character
            .sprite
            .animations
            .to_vec()
            .into_iter()
            .map(|a| a.into())
            .collect::<Vec<_>>();

        let texture_id = character.sprite.texture_id.clone();

        let params = {
            let meta: AnimatedSpriteMetadata = character.sprite.clone().into();

            AnimatedSpriteParams {
                offset,
                ..meta.into()
            }
        };

        let sprites = vec![(
            BODY_ANIMATED_SPRITE_ID,
            AnimatedSprite::new(&texture_id, animations.as_slice(), params),
        )];

        let draw_order = (index as u32 + 1) * 10;

        let size = character.collider_size.as_i32();
        let actor = storage::get_mut::<CollisionWorld>().add_actor(position, size.x, size.y);

        let body_params = PhysicsBodyParams {
            offset: vec2(-character.collider_size.x / 2.0, 0.0),
            size: character.collider_size,
            has_friction: false,
            can_rotate: false,
            ..Default::default()
        };

        world.spawn((
            Player::new(index, position),
            Transform::from(position),
            PlayerController::from(controller),
            PlayerAttributes::from(&character),
            PlayerInventory::from(weapon_mount),
            PlayerEventQueue::new(),
            Drawable::new_animated_sprite_set(draw_order, &sprites),
            PhysicsBody::new(actor, None, body_params),
        ));
    }
}