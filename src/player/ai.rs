use hecs::World;
use macroquad::prelude::collections::storage;

use crate::GameInput;

struct Condition{
    function: Box<dyn Fn(i32) -> bool + Send + Sync>
}

impl std::fmt::Debug for Condition{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

trait BehaviourTreeNode/* : Send + Sync + std::fmt::Debug*/{
    fn evaluate(&mut self, world: &World, input: &mut GameInput) -> Option<bool>;
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
    fn evaluate(&mut self, world: &World, input: &mut GameInput) -> Option<bool> {
        if self.index.is_none(){ //If not running, start from beginning
            self.index = Some(0);
        }
        match self.children[self.index.unwrap()].evaluate(world, input){ //Eval next node in line
            Some(successful) => { // When a child finishes running...
                self.index = Some(self.index.unwrap() + 1); // ...go to next

                if !successful && self.return_on_fail{
                    self.index = None;
                    Some(false) // Not all have run, return failure
                }else if self.index.unwrap() == self.children.len(){
                    self.index = None;
                    Some(true)  // All children have at least tried to run
                }else{
                    None // Run next node
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
    
    condition: Box<dyn Fn(&World) -> bool>,
    child: Box<dyn BehaviourTreeNode>,
    running: bool,
    reevaluate: bool
}

impl ConditionNode{
    fn create(condition: Box<dyn Fn(&World) -> bool>, child: Box<dyn BehaviourTreeNode>, reevaluate: bool) -> Box<ConditionNode>{
        Box::new(ConditionNode{
            condition,
            child,
            running: false,
            reevaluate,
        })
    }
}

impl BehaviourTreeNode for ConditionNode{
    fn evaluate(&mut self, world: &World, input: &mut GameInput) -> Option<bool> {
        if self.reevaluate || !self.running{
            //if(true){
            if (self.condition)(world){
            
                self.running = true;
                self.child.start();
            }else{
                return Some(false)
            }
        }
        match self.child.evaluate(world, input){
            Some(success) => {
                self.running = false;
                Some(success)
            },
            None => None,
        }
    }

    // fn safe_clone(&self) -> std::boxed::Box<(dyn BehaviourTreeNode + 'static)>{
    //     Box::new(self.clone())
    // }
}

// #[derive(Debug, Clone)]
pub struct Ai {
    jump_cooldown: f32,
    throw_cooldown: f32,
    keep_direction_until_event: bool,
    keep_direction_timeout: f32,
    fix_direction: i32,
    tree: Box<dyn BehaviourTreeNode>
}

/*impl<T: BehaviourTreeNode> std::fmt::Debug for T{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        &self.
    }
}*/

 

impl Ai {
    pub fn create() -> usize {
        let mut ais = storage::get_mut::<Vec<Ai>>();
        ais.push(
            Ai {
                jump_cooldown: 0.,
                throw_cooldown: 0.,
                keep_direction_until_event: false,
                keep_direction_timeout: 0.,
                fix_direction: 0,
                tree: Sequence::new(vec![
                    Box::new(ConditionNode{ 
                        condition: todo!(), 
                        child: todo!(), 
                        running: todo!(), 
                        reevaluate: todo!() })
                ], true)
            }
        );
        ais.len() - 1
    }

    pub fn update(&mut self, world: &World) -> GameInput {
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
        self.tree.evaluate(&world, &mut i);

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

// impl Default for Ai {
//     fn default() -> Self {
//         Self::new()
//     }
// }
