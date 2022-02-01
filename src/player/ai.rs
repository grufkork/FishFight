use hecs::World;

use crate::GameInput;

enum goal{
    GetWeapon,

}

trait BehaviourTreeNode{
    fn evaluate(&mut self) -> Option<bool>;
    fn start(&mut self) {}
}

struct Sequence{
    children: Vec<Box<dyn BehaviourTreeNode>>,
    index: Option<usize>,
    return_on_fail: bool
}

impl Sequence{
    fn new(children: Vec<Box<dyn BehaviourTreeNode>>, return_on_fail: bool) -> Box<dyn BehaviourTreeNode>{
        Box::new(Sequence{
            children,
            index: None,
            return_on_fail,
        })
    }
}

impl BehaviourTreeNode for Sequence{
    fn evaluate(&mut self) -> Option<bool> {
        if self.index.is_none(){
            self.index = Some(0);
        }
        match self.children[self.index.unwrap()].evaluate(){
            Some(success) => {
                self.index = Some(self.index.unwrap() + 1);
                if success{
                    if self.index.unwrap() == self.children.len() {
                        self.index = None;
                        Some(true)
                    }else{
                        self.children[self.index.unwrap()].start();
                        None
                    }
                }else if self.return_on_fail{
                    self.index = None;
                    Some(false)
                }else{
                    self.children[self.index.unwrap()].start();
                    None
                }
            },
            None => None,
        }
    }
}

struct ConditionNode{
    condition: Box<dyn Fn(i32) -> bool>,
    child: Box<dyn BehaviourTreeNode>,
    running: bool
}

impl BehaviourTreeNode for ConditionNode{
    fn evaluate(&mut self) -> Option<bool> {
        if !self.running{
            if (self.condition)(1337){
                self.running = true;
                self.child.start();
            }else{
                return Some(false)
            }
        }
        match self.child.evaluate(){
            Some(success) => {
                self.running = false;
                Some(success)
            },
            None => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ai {
    jump_cooldown: f32,
    throw_cooldown: f32,
    keep_direction_until_event: bool,
    keep_direction_timeout: f32,
    fix_direction: i32,
    tree: Box<dyn BehaviourTreeNode>
}

impl Ai {
    pub fn new() -> Ai {
        Ai {
            jump_cooldown: 0.,
            keep_direction_until_event: false,
            keep_direction_timeout: 0.,
            fix_direction: 0,
            throw_cooldown: 0.,
            tree: Sequence::new(vec![

            ], true)
        }
    }

    pub fn update(&mut self, world: &mut World) -> GameInput {
        let input = GameInput {
            right: false,
            left: true,
            down: false,
            jump: true,
            float: false,
            pickup: false,
            fire: false,
            slide: false,
        };

        /*
        let foe = scene::find_nodes_by_type::<OldPlayer>().next().unwrap();

        let mut following_horiz = false;

        if (player.body.position.x - foe.body.position.x).abs() >= 50. {
            //
            if !self.keep_direction_until_event {
                following_horiz = true;
                if player.body.position.x > foe.body.position.x {
                    input.left = true;
                } else {
                    input.right = true;
                }
            }
        }

        if !self.keep_direction_until_event
            && (player.body.position.y - foe.body.position.y).abs() >= 50.
            && !following_horiz
        {
            self.fix_direction = if rand::gen_range(0, 2) == 0 { 1 } else { -1 };
            self.keep_direction_until_event = true;
        }

        let dir = if input.left {
            -1.
        } else if input.right {
            1.
        } else {
            0.
        };

        {
            let collision_world = &mut storage::get_mut::<GameWorld>().collision_world;

            let obstacle_soon = collision_world.collide_check(
                player.body.collider,
                player.body.position + vec2(15. * dir, 0.),
            );
            let cliff_soon = !collision_world.collide_check(
                player.body.collider,
                player.body.position + vec2(5. * dir, 5.),
            );
            let wants_descent = player.body.position.y < foe.body.position.y;

            if (cliff_soon || obstacle_soon) && self.keep_direction_timeout <= 0. {
                self.keep_direction_until_event = false;
                self.fix_direction = 0;
                self.keep_direction_timeout = 1.;
            }

            if (obstacle_soon || (!wants_descent && cliff_soon))
                && player.body.is_on_ground
                && self.jump_cooldown <= 0.
            {
                input.jump = true;
                self.jump_cooldown = 0.2;
            }
        }

        if rand::gen_range(0, 200) == 5 {
            self.fix_direction = if rand::gen_range(0, 2) == 0 { 1 } else { -1 };
            self.keep_direction_until_event = true;
        }

        if rand::gen_range(0, 800) == 5 {
            input.pickup = true;
            self.throw_cooldown = 1.;
        }

        if player.body.position.distance(foe.body.position) <= 100. || rand::gen_range(0, 180) == 5
        {
            //
            if player.state_machine.state() == OldPlayer::ST_NORMAL && player.weapon.is_some() {
                player.state_machine.set_state(OldPlayer::ST_ATTACK);
            }
        }

        if self.jump_cooldown >= 0. {
            self.jump_cooldown -= get_frame_time();
        }
        if self.throw_cooldown >= 0. {
            self.throw_cooldown -= get_frame_time();
        }

        if self.keep_direction_timeout >= 0. {
            self.keep_direction_timeout -= get_frame_time();
        }

        if self.throw_cooldown <= 0.0 {
            for item in scene::find_nodes_by_type::<MapItem>() {
                let item_collider = item.body.get_collider_rect();
                if item_collider.point().distance(player.body.position) <= 80. {
                    input.pickup = true;
                }
            }
            self.throw_cooldown = 1.;
        }

         */

        input
    }
}

impl Default for Ai {
    fn default() -> Self {
        Self::new()
    }
}
