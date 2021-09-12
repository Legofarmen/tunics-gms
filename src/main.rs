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

impl<T, O, L> Room<T, O, L> {}

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

pub trait Ev: Clone {
    type Room: Ro;

    fn apply(&self, room: &mut Self::Room) -> bool;
}

pub trait Ro: Sized {
    fn with_exits(exits: Vec<Self>) -> Self;
}

fn room_tree<E: Clone + Ev>(tree: Tree<E>) -> E::Room {
    struct V;

    impl<E: Ev> Visitor<E> for V {
        type Room = E::Room;

        fn visit_event(&mut self, event: &E, next: Tree<E>) -> Self::Room {
            let mut room = next.accept(self);
            if event.apply(&mut room) {
                room
            } else {
                let mut room = Self::Room::with_exits(vec![room]);
                event.apply(&mut room);
                room
            }
        }

        fn visit_branch(&mut self, nodes: Vec<Tree<E>>) -> E::Room {
            let nodes: Vec<_> = nodes.into_iter().map(|node| node.accept(self)).collect();
            E::Room::with_exits(nodes)
        }
    }
    tree.accept(&mut V)
}

impl Ro for Room<Treasure, Obstacle, Lock> {
    fn with_exits(exits: Vec<Self>) -> Self {
        Room {
            entrance: None,
            exits,
            chest: None,
            obstacle: None,
            far_side_chest: None,
        }
    }
}

impl Ev for Event {
    type Room = Room<Treasure, Obstacle, Lock>;

    fn apply(&self, room: &mut Room<Treasure, Obstacle, Lock>) -> bool {
        match self {
            Event::Boss
                if room.entrance.is_none() && room.chest.is_none() && room.obstacle.is_none() =>
            {
                room.obstacle = Some(Obstacle::Boss);
                room.entrance = Some(Lock::BigKey);
                true
            }
            Event::Obstacle(obstacle) if room.entrance.is_none() && room.obstacle.is_none() => {
                room.obstacle = Some(*obstacle);
                if room.chest.is_some() {
                    room.far_side_chest = Some(true);
                }
                true
            }
            Event::SmallChest(treasure) if room.entrance.is_none() && room.chest.is_none() => {
                room.chest = Some(*treasure);
                if room.obstacle.is_some() {
                    room.far_side_chest = Some(false);
                }
                true
            }
            Event::HiddenSmallChest(treasure)
                if room.entrance.is_none() && room.chest.is_none() && room.obstacle.is_none() =>
            {
                room.chest = Some(*treasure);
                room.obstacle = Some(Obstacle::Puzzle);
                true
            }
            Event::BigChest(treasure)
                if room.entrance.is_none() && room.chest.is_none() && room.obstacle.is_none() =>
            {
                room.chest = Some(*treasure);
                room.obstacle = Some(Obstacle::BigChest);
                true
            }
            Event::HideSmallChests => {
                unimplemented!("should removed prior to this");
            }
            Event::SmallKeyDoor if room.entrance.is_none() => {
                room.entrance = Some(Lock::SmallKey);
                true
            }
            Event::Entrance
                if room.entrance.is_none() && room.chest.is_none() && room.obstacle.is_none() =>
            {
                room.obstacle = Some(Obstacle::Entrance);
                true
            }
            Event::CulDeSac
                if room.entrance.is_none() && room.chest.is_none() && room.obstacle.is_none() =>
            {
                *room = Room::with_exits(vec![room.clone()]);
                room.obstacle = Some(Obstacle::CulDeSac);
                true
            }
            Event::Fairy
                if room.entrance.is_none() && room.chest.is_none() && room.obstacle.is_none() =>
            {
                *room = Room::with_exits(vec![room.clone()]);
                room.obstacle = Some(Obstacle::Fairy);
                true
            }
            _ => false,
        }
    }
}

fn main() {
    use crate::tunics::calc_join_weight;
    use crate::tunics::hide_chests;
    use crate::tunics::OutlineConf;
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
