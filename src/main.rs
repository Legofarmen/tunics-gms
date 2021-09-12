pub mod event_tree;
pub mod outline;
pub mod tunics;

use crate::event_tree::Tree;
use crate::event_tree::Visitor;
use crate::tunics::Event;
use crate::tunics::Obstacle;
use crate::tunics::Treasure;

#[derive(Clone, Debug)]
pub enum Lock {
    SmallKey,
    BigKey,
    Full,
}

#[derive(Clone)]
pub struct Room<T, O, L> {
    pub entrance: Option<L>,
    pub exits: Vec<Room<T, O, L>>,
    pub chest: Option<T>,
    pub obstacle: Option<O>,
    pub far_side_chest: Option<bool>,
}

impl<T, O, L> Default for Room<T, O, L> {
    fn default() -> Self {
        Room {
            entrance: None,
            exits: Vec::new(),
            chest: None,
            obstacle: None,
            far_side_chest: None,
        }
    }
}

impl<T, O, L> Room<T, O, L> {
    fn with_exits(exits: Vec<Room<T, O, L>>) -> Self {
        Room {
            entrance: None,
            exits,
            chest: None,
            obstacle: None,
            far_side_chest: None,
        }
    }
}

impl<T, O, L> Room<T, O, L>
where
    T: std::fmt::Debug,
    O: std::fmt::Debug,
    L: std::fmt::Debug,
{
    fn show(&self) {
        fn visit<T, O, L>(indent: usize, room: &Room<T, O, L>)
        where
            T: std::fmt::Debug,
            O: std::fmt::Debug,
            L: std::fmt::Debug,
        {
            let lock = if let Some(lock) = &room.entrance {
                format!("{:?}", lock)
            } else {
                "".to_string()
            };
            let side = match room.far_side_chest {
                Some(true) => "beyond",
                Some(false) => "in front of",
                None => "",
            };
            println!(
                "{:indent$}* {}-{}-{}-{}",
                "",
                lock,
                room.chest
                    .as_ref()
                    .map(|v| format!("{:?}", v))
                    .unwrap_or("".to_string()),
                side,
                room.obstacle
                    .as_ref()
                    .map(|v| format!("{:?}", v))
                    .unwrap_or("".to_string()),
                indent = indent
            );
            for exit in &room.exits {
                visit(indent + 2, exit);
            }
        }
        visit(0, self);
    }
}

struct V;

impl Visitor<Event> for V {
    type Room = Room<Treasure, Obstacle, Lock>;

    fn visit_event(&mut self, event: &Event, next: Tree<Event>) -> Self::Room {
        let mut room = next.accept(self);
        event.apply(&mut room);
        room
    }

    fn visit_branch(&mut self, nodes: Vec<Tree<Event>>) -> Self::Room {
        Room::with_exits(nodes.into_iter().map(|node| node.accept(self)).collect())
    }
}

fn room_tree(tree: Tree<Event>) -> Room<Treasure, Obstacle, Lock> {
    tree.accept(&mut V)
}

fn main() {
    use crate::event_tree::Tree;
    use crate::tunics::calc_join_weight;
    use crate::tunics::hide_chests;
    use crate::tunics::OutlineConf;
    use crate::tunics::Treasure;
    use rand::SeedableRng;
    let mut rng = rand::rngs::StdRng::seed_from_u64(3);
    //let mut rng = rand::rngs::ThreadRng::default();

    let outline = OutlineConf {
        num_fairies: 0,
        num_cul_de_sacs: 0,
        num_small_keys: 0,
        //treasures: [Treasure::BombsCounter].iter().cloned().collect(),
        treasures: [].iter().cloned().collect(),
    }
    .into_outline();
    let actions = outline.action_sequence(&mut rng);

    /*
    outline.show();
    for action in &actions {
        println!("{:?}", action);
    }
    */

    let mut tree = Tree::from_actions(&mut rng, 3, &actions, calc_join_weight);
    hide_chests(&mut rng, &mut tree);
    tree.show();

    let room = room_tree(tree);
    room.show();
}
