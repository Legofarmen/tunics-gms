use crate::event_tree::Action as EventAction;
use crate::event_tree::Tree;
use crate::outline::Outline;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Obstacle {
    Entrance,
    WeakWall,
    VeryWeakWall,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Treasure {
    BigKey,
    BombsCounter,
    Map,
    Compass,
    SmallKey,
}

impl Treasure {
    fn get_obstacles(self) -> &'static [Obstacle] {
        match self {
            Treasure::BigKey => &[],
            Treasure::BombsCounter => &[Obstacle::WeakWall, Obstacle::VeryWeakWall],
            Treasure::Map => &[],
            Treasure::Compass => &[],
            Treasure::SmallKey => &[],
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Action {
    Boss,
    Chest(Treasure),
    BigChest(Treasure),
    LockedDoor,
    Fairy,
    CulDeSac,
    Obstacle(Obstacle),
    HideChests,
}

impl EventAction for Action {
    type Item = ();
    fn apply(&self, _heads: &mut Vec<Tree<()>>) {}
}

pub fn calc_join_weight(_tree: &Tree<()>) -> Box<dyn Fn(&Tree<()>) -> u32> {
    Box::new(|_| 0)
}

pub struct OutlineConf {
    pub num_small_keys: usize,
    pub num_fairies: usize,
    pub num_cul_de_sacs: usize,
    pub treasures: HashSet<Treasure>,
}

impl OutlineConf {
    pub fn into_outline(self) -> Outline<Action> {
        let mut outline = Outline::new();
        let entrance = outline.node(Action::Obstacle(Obstacle::Entrance));

        for _ in 0..self.num_cul_de_sacs {
            let cul_de_sac = outline.node(Action::CulDeSac);
            outline.dep(entrance, cul_de_sac);
        }

        for _ in 0..self.num_fairies {
            let fairy = outline.node(Action::Fairy);
            outline.dep(entrance, fairy);
        }

        let boss = outline.node(Action::Boss);
        let big_key = outline.node(Action::Chest(Treasure::BigKey));
        outline.dep(big_key, boss);

        let hide_chests = outline.node(Action::HideChests);
        let compass = outline.node(Action::Chest(Treasure::Compass));
        outline.dep(hide_chests, boss);
        outline.dep(compass, hide_chests);
        outline.dep(entrance, compass);

        for treasure in &self.treasures {
            let big_chest = outline.node(Action::BigChest(*treasure));
            outline.dep(big_key, big_chest);
            for obstacle in treasure.get_obstacles() {
                let obstacle = outline.node(Action::Obstacle(*obstacle));
                outline.dep(big_chest, obstacle);
                outline.dep(obstacle, boss);
            }
        }

        let mut last_locked_door = None;
        for i in 0..self.num_small_keys {
            let locked_door = outline.node(Action::LockedDoor);
            if let Some(last_locked_door) = last_locked_door {
                outline.dep(locked_door, last_locked_door);
            } else {
                outline.dep(locked_door, big_key);
            }
            if i == self.num_small_keys - 1 {
                let mut last_small_key = None;
                for j in 0..self.num_small_keys {
                    let small_key = outline.node(Action::Chest(Treasure::SmallKey));
                    if let Some(last_small_key) = last_small_key {
                        outline.dep(small_key, last_small_key);
                    } else {
                        outline.dep(small_key, locked_door);
                    }
                    if j == self.num_small_keys - 1 {
                        outline.dep(entrance, small_key);
                    }
                    last_small_key = Some(small_key);
                }
            }
            last_locked_door = Some(locked_door);
        }
        if self.num_small_keys == 0 {
            outline.dep(entrance, big_key);
        }

        let map = outline.node(Action::Chest(Treasure::Map));
        if let Some(weak_wall) = outline.index(Action::Obstacle(Obstacle::WeakWall)) {
            let very_weak_wall = outline
                .index(Action::Obstacle(Obstacle::VeryWeakWall))
                .unwrap();
            outline.dep(very_weak_wall, weak_wall);
            outline.dep(map, very_weak_wall);
        } else {
            outline.dep(map, boss);
        }
        outline.dep(entrance, map);

        outline
    }
}
