pub mod event_tree;
pub mod outline;
pub mod tunics;

use crate::event_tree::Tree;
use crate::tunics::Event;
use crate::tunics::Lock;
use crate::tunics::Obstacle;
use crate::tunics::Room;
use crate::tunics::Treasure;

use crate::event_tree::Ev;
use crate::event_tree::Ro;

impl Ro for Room<Treasure, Obstacle, Lock> {
    fn add_exits<I>(mut self, exits: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        self.exits.extend(exits);
        self
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
                *room = Room::default().add_exits(vec![room.clone()]);
                room.obstacle = Some(Obstacle::CulDeSac);
                true
            }
            Event::Fairy
                if room.entrance.is_none() && room.chest.is_none() && room.obstacle.is_none() =>
            {
                *room = Room::default().add_exits(vec![room.clone()]);
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

    let room = tree.room_tree();
    room.show();
}
