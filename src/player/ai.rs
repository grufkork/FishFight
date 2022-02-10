use hecs::{World, Entity};
use macroquad::prelude::collections::storage;
use macroquad_platformer::Tile;

use crate::{GameInput, items::Weapon, Transform, CollisionWorld};

use super::{Player, PlayerInventory};

struct Condition{
    function: Box<dyn Fn(i32) -> bool + Send + Sync>
}

impl std::fmt::Debug for Condition{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

trait BehaviourTreeNode/* : Send + Sync + std::fmt::Debug*/{
    fn evaluate(&mut self, world: &World, ai: &Ai, input: &mut GameInput) -> Option<bool>;
    fn start(&mut self) {}
    // fn safe_clone(&self) -> Box<dyn BehaviourTreeNode>;
}

/*impl Clone for Box<dyn BehaviourTreeNode>{
    fn clone(&self) -> Box<dyn BehaviourTreeNode> {
        self.safe_clone()
    }
}*/

// impl Clone for Box<dyn BehaviourTreeNode>{
//     fn clone(&self) -> Self{
//         self.safe_clone()
//     }
// }

trait CloneWrap: Clone{

}

// #[derive(Debug, Clone)]
struct Sequence{
    children: Vec<Box<dyn BehaviourTreeNode>>,
    index: Option<usize>,
    return_on_success: bool
}

impl Sequence{
    fn create(return_on_success: bool, children: Vec<Box<dyn BehaviourTreeNode>>) -> Box<dyn BehaviourTreeNode>{
        Box::new(Sequence{
            children,
            index: None,
            return_on_success,
        })
    }
}

impl BehaviourTreeNode for Sequence{
    fn evaluate(&mut self, world: &World, ai: &Ai, input: &mut GameInput) -> Option<bool> {
        if self.index.is_none(){ //If not running, start from beginning
            self.index = Some(0);
        }
        match self.children[self.index.unwrap()].evaluate(world, ai, input){ //Eval next node in line
            Some(successful) => { // When a child finishes running...
                self.index = Some(self.index.unwrap() + 1); // ...go to next

                if successful && self.return_on_success{
                    self.index = None;
                    Some(true) // One has succeeded, return
                }else if self.index.unwrap() == self.children.len(){
                    self.index = None;
                    Some(!self.return_on_success)  // All children have at tried to run. If it's supposed to return on success, this means it is a failure.
                }else{
                    self.evaluate(world, ai, input) // Run next node immediately
                }
            },
            None => None, //If child is still running, this one is running too
        }
    }

    // fn safe_clone(&self) -> std::boxed::Box<(dyn BehaviourTreeNode + 'static)>{
    //     Box::new(self.clone())
    // }
}

// #[derive(Debug, Clone)]
struct ConditionNode{
    //condition: Condition,
    
    condition: Box<dyn Fn(&World, &Ai) -> bool>,
    child: Box<dyn BehaviourTreeNode>,
    running: bool,
    reevaluate: bool
}

impl ConditionNode{
    fn create(reevaluate: bool, condition: Box<dyn Fn(&World, &Ai) -> bool>, child: Box<dyn BehaviourTreeNode>) -> Box<ConditionNode>{
        Box::new(ConditionNode{
            condition,
            child,
            running: false,
            reevaluate,
        })
    }
}

impl BehaviourTreeNode for ConditionNode{
    fn evaluate(&mut self, world: &World, ai: &Ai, input: &mut GameInput) -> Option<bool> {
        if self.reevaluate || !self.running{ // Will only check if child is not running, or it is supposed to recheck every time
            //if(true){
            if (self.condition)(world, ai){
            
                self.running = true;
                self.child.start();
            }else{
                return Some(false)
            }
        }
        match self.child.evaluate(world, ai, input){
            Some(success) => {
                self.running = false;
                Some(success)
            },
            None => None,
        }
    }
}

struct Inverter{
    child: Box<dyn BehaviourTreeNode>
}

impl Inverter{
    fn create(child: Box<dyn BehaviourTreeNode>) -> Box<Inverter>{
        Box::new(
            Inverter{
                child
            }
        )
    }
}

impl BehaviourTreeNode for Inverter{
    fn evaluate(&mut self, world: &World, ai: &Ai, input: &mut GameInput) -> Option<bool> {
        self.child.evaluate(world, ai, input).map(|success| !success)
    }
}

struct Behaviour{
    action: Box<dyn Fn(&World, &Ai, &mut GameInput) -> Option<bool>>
}

impl Behaviour{
    fn create(action: Box<dyn Fn(&World, &Ai, &mut GameInput) -> Option<bool>>) -> Box<Behaviour>{
        Box::new(
            Behaviour{
                action
            }
        )
    }
}

impl BehaviourTreeNode for Behaviour{
    fn evaluate(&mut self, world: &World, ai: &Ai, input: &mut GameInput) -> Option<bool> {
        (self.action)(world, ai, input)
    }
}
pub struct Ai {
    jump_cooldown: f32,
    throw_cooldown: f32,
    keep_direction_until_event: bool,
    keep_direction_timeout: f32,
    fix_direction: i32,
    tree: Option<Box<dyn BehaviourTreeNode>>,
    entity: Option<Entity>,
    player_id: u8,
    ai_id: usize
}

/*impl<T: BehaviourTreeNode> std::fmt::Debug for T{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        &self.
    }
}*/

 

impl Ai {
    pub fn create(player_id: u8) -> usize {
        let mut ais = storage::get_mut::<Vec<Ai>>();

        let has_weapon_condition= |world: &World, ai: &Ai| {
            world.query_one::<(&PlayerInventory, &Player)>(ai.entity.unwrap()).unwrap().get().unwrap().0.weapon.is_some()

            /*let mut players = world.query::<(&crate::Transform, &super::Player, &super::PlayerController, &super::PlayerInventory)>();
            
            let player = players.iter().find(|(e, (t, p, c, i))| {p.index == ai.player_id}).unwrap();
            
            player.1.3.weapon.is_some()*/
        };

        let ai_id = ais.len();

        ais.push(
            Ai {
                jump_cooldown: 0.,
                throw_cooldown: 0.,
                keep_direction_until_event: false,
                keep_direction_timeout: 0.,
                fix_direction: 0,

                tree: Some(Sequence::create(
                    true,
                    vec![
                        ConditionNode::create( // If has weapon, try to shoot enemy
                            true,
                            Box::new(has_weapon_condition),
                            Sequence::create(
                                true,
                                vec![
                                    //ConditionNode::create(condition, child, reevaluate)
                                    Behaviour::create(
                                        Box::new(|world, ai, input|{
                                            let mut player = world.query_one::<(&Transform, &Player)>(ai.entity.unwrap()).unwrap();
                                            let player = player.get().unwrap();

                                            let mut e = world.query::<(&Transform, &Player)>();
                                            let mut enemies = e.iter().filter(|(_, (t, p))| {
                                                (t.position.y - player.0.position.y).abs() < 20. && player.1.index != p.index && p.respawn_timer == 0.0
                                            });

                                            let collision = &mut storage::get_mut::<CollisionWorld>();

                                            Some(enemies.find(|(_, (t, _))|{
                                                let average = (player.0.position + t.position) /2.0;
                                                let diff = (player.0.position - t.position);
                                                if collision.collide_solids(average, diff.x.abs() as i32, 5) == Tile::Empty{
                                                    if diff.x > 0.{
                                                        input.left = true;
                                                    }else{
                                                        input.right = true;
                                                    }
                                                    input.fire = true;
                                                    true
                                                }else{
                                                    false
                                                }
                                            }).is_none())
                                        })
                                    ),
                                    Behaviour::create(
                                        Box::new(|world, ai, input|{
                                            // Wander
                                            Some(true)
                                        })
                                    )
                                ]
                            ),
                        ),
                        Sequence::create( // Get a weapon
                            true,
                            vec![
                                Inverter::create(Behaviour::create(Box::new(|world, ai, input|{
                                    let mut p = world.query_one::<(&Transform)>(ai.entity.unwrap()).unwrap();
                                    let player = p.get().unwrap();
                                    //let player = world::query_one()

                                    let mut min_d = 999999.0;
                                    let mut weapons_source = world.query::<(&Transform, &Weapon)>();
                                    let mut target = None;
                                    /*match weapons.next() {
                                        Some(val) => val.1,
                                        None => return Some(false),
                                    };*/
                                    for (i, w) in weapons_source.iter(){
                                        let diff = w.0.position - player.position;
                                        //diff.x = diff.x.abs();
                                        if diff.x.abs() + diff.y.abs()*3.0 < min_d{
                                            min_d = diff.x.abs() + diff.y.abs() * 3.0;
                                            target = Some(w);
                                        }
                                    }

                                    if target.is_none() {
                                        return Some(false)
                                    }

                                    if (target.unwrap().0.position.x - player.position.x).abs() < 2.{
                                        Some(true)
                                    }else {
                                        if target.unwrap().0.position.x < player.position.x {
                                            input.left = true;
                                        }else{
                                            input.right = true; 
                                        }
                                        None
                                    }



                                    //let target = weapons.sort_by|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal)).next();
                                }))),
                                Behaviour::create(Box::new(|world, ai, input|{
                                    input.pickup = true;
                                    Some(true)
                                }))
                            ]
                        )
                    ]
                )),
                player_id,
                ai_id,
                entity: None
            }
        );
        ai_id
    }

    pub fn update(&mut self, world: &World) -> GameInput {
        if self.entity.is_none(){
            let mut playerlikes = world.query::<&super::Player>();
            self.entity = Some(playerlikes.iter().find(|(e, p)| {p.index == self.player_id}).unwrap().0);
        }

        let mut i = GameInput{
            left: false,
            right: false,
            down: false,
            jump: false,
            float: false,
            pickup: false,
            fire: false,
            slide: false,
        };

        let mut tree = self.tree.take().unwrap();
        tree.evaluate(world, self, &mut i);

        self.tree = Some(tree);

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

        i
    }
}