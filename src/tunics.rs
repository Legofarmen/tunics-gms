use crate::event_tree::Action;
use crate::event_tree::Tree;
use crate::outline::Outline;
use rand::Rng;
use std::collections::HashSet;

const NODE_DEPTH_WEIGHT: usize = 1;
const BIG_KEY_DEPTH_WEIGHT: usize = 2;

pub struct OutlineConf {
    pub num_small_keys: usize,
    pub num_fairies: usize,
    pub num_cul_de_sacs: usize,
    pub treasures: HashSet<Treasure>,
}

impl OutlineConf {
    pub fn into_outline(self) -> Outline<Event> {
        let mut outline = Outline::new();
        let entrance = outline.node(Action::PrependGrouped(Event::Entrance));

        for _ in 0..self.num_cul_de_sacs {
            let cul_de_sac = outline.node(Action::AddEvent(Event::CulDeSac));
            outline.dep(entrance, cul_de_sac);
        }

        for _ in 0..self.num_fairies {
            let fairy = outline.node(Action::AddEvent(Event::Fairy));
            outline.dep(entrance, fairy);
        }

        let boss = outline.node(Action::AddEvent(Event::Boss));
        let big_key = outline.node(Action::AddEvent(Event::SmallChest(Treasure::BigKey)));
        outline.dep(big_key, boss);

        let hide_chests = outline.node(Action::PrependEach(Event::HideSmallChests));
        let compass = outline.node(Action::AddEvent(Event::SmallChest(Treasure::Compass)));
        outline.dep(hide_chests, boss);
        outline.dep(compass, hide_chests);
        outline.dep(entrance, compass);

        for treasure in &self.treasures {
            let big_chest = outline.node(Action::AddEvent(Event::BigChest(*treasure)));
            outline.dep(big_key, big_chest);
            for obstacle in treasure.get_obstacles() {
                let obstacle = outline.node(Action::PrependEach(Event::Obstacle(*obstacle)));
                outline.dep(big_chest, obstacle);
                outline.dep(obstacle, boss);
            }
        }

        let mut last_locked_door = None;
        for i in 0..self.num_small_keys {
            let locked_door = outline.node(Action::PrependAny(Event::SmallKeyDoor));
            if let Some(last_locked_door) = last_locked_door {
                outline.dep(locked_door, last_locked_door);
            } else {
                outline.dep(locked_door, big_key);
            }
            if i == self.num_small_keys - 1 {
                let mut last_small_key = None;
                for j in 0..self.num_small_keys {
                    let small_key =
                        outline.node(Action::AddEvent(Event::SmallChest(Treasure::SmallKey)));
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

        let map = outline.node(Action::AddEvent(Event::SmallChest(Treasure::Map)));
        if let Some(weak_wall) =
            outline.index(Action::PrependEach(Event::Obstacle(Obstacle::WeakWall)))
        {
            let very_weak_wall = outline
                .index(Action::PrependEach(Event::Obstacle(Obstacle::VeryWeakWall)))
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

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Obstacle {
    Entrance,
    WeakWall,
    VeryWeakWall,
    Boss,
    Puzzle,
    BigChest,
    Fairy,
    CulDeSac,
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Event {
    Boss,
    CulDeSac,
    Fairy,
    Obstacle(Obstacle),
    SmallChest(Treasure),
    HiddenSmallChest(Treasure),
    BigChest(Treasure),
    HideSmallChests,
    SmallKeyDoor,
    Entrance,
}

impl Event {}

pub fn calc_join_weight(
    tree: &Tree<Event>,
    global_max_depth: usize,
) -> Box<dyn Fn(&Tree<Event>) -> usize> {
    fn big_key_pred(event: &Event) -> bool {
        matches!(
            event,
            Event::Boss | Event::BigChest(_) | Event::SmallChest(Treasure::BigKey)
        )
    }
    let depth = tree.find_event_depth(&big_key_pred);
    if depth.is_some() {
        Box::new(move |node| {
            let node_max_depth = node.max_depth();
            let big_key_depth = node
                .find_event_depth(&big_key_pred)
                .unwrap_or(global_max_depth);
            NODE_DEPTH_WEIGHT * (global_max_depth - node_max_depth)
                + BIG_KEY_DEPTH_WEIGHT * big_key_depth
                + 1
        })
    } else {
        Box::new(move |node| {
            let node_max_depth = node.max_depth();
            let big_key_depth = node
                .find_event_depth(&big_key_pred)
                .unwrap_or(global_max_depth);
            NODE_DEPTH_WEIGHT * (global_max_depth - node_max_depth)
                + BIG_KEY_DEPTH_WEIGHT * (global_max_depth - big_key_depth)
                + 1
        })
    }
}

pub fn hide_chests<R: Rng>(rng: &mut R, tree: &mut Tree<Event>) {
    fn visit<R: Rng>(rng: &mut R, tree: &mut Tree<Event>, is_below: bool) {
        match tree {
            Tree::Event(Event::HideSmallChests, next) => {
                visit(rng, next, true);
                *tree = (**next).clone();
            }
            Tree::Event(event @ Event::SmallChest(_), next) if is_below => {
                let treasure = if let Event::SmallChest(treasure) = event {
                    *treasure
                } else {
                    unreachable!()
                };
                if rng.gen_bool(0.5) {
                    *event = Event::HiddenSmallChest(treasure);
                }
                visit(rng, next, true);
            }
            Tree::Event(_, next) => visit(rng, next, is_below),
            Tree::Branch(nodes) => {
                for node in nodes {
                    visit(rng, node, is_below);
                }
            }
        }
    }
    visit(rng, tree, false)
}
