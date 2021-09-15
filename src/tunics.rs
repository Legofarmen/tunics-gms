use crate::event_tree;
use crate::event_tree::Action;
use crate::event_tree::Transform;
use crate::event_tree::Tree;
use crate::outline::Outline;
use rand::distributions::weighted::WeightedIndex;
use rand::distributions::Distribution;
use rand::Rng;
use std::collections::HashSet;

const NODE_DEPTH_WEIGHT: usize = 1;
const BIG_KEY_DEPTH_WEIGHT: usize = 2;

pub struct Config {
    pub num_small_keys: usize,
    pub num_fairies: usize,
    pub num_cul_de_sacs: usize,
    pub treasures: HashSet<Treasure>,
}

impl From<Config> for Outline<Feature, HideSmallChests> {
    fn from(config: Config) -> Outline<Feature, HideSmallChests> {
        let mut outline = Outline::new();
        let entrance = outline.node(Action::PrependGrouped(Feature::Entrance));

        for _ in 0..config.num_cul_de_sacs {
            let cul_de_sac = outline.node(Action::New(Feature::CulDeSac));
            outline.dep(entrance, cul_de_sac);
        }

        for _ in 0..config.num_fairies {
            let fairy = outline.node(Action::New(Feature::Fairy));
            outline.dep(entrance, fairy);
        }

        let boss = outline.node(Action::New(Feature::Boss));
        let big_key = outline.node(Action::New(Feature::SmallChest(Treasure::BigKey)));
        outline.dep(big_key, boss);

        let hide_chests = outline.node(Action::TransformEach(HideSmallChests));
        let compass = outline.node(Action::New(Feature::SmallChest(Treasure::Compass)));
        outline.dep(hide_chests, boss);
        outline.dep(compass, hide_chests);
        outline.dep(entrance, compass);

        for treasure in &config.treasures {
            let big_chest = outline.node(Action::New(Feature::BigChest(*treasure)));
            outline.dep(big_key, big_chest);
            for obstacle in treasure.get_obstacles() {
                let obstacle = outline.node(Action::PrependEach(Feature::Obstacle(*obstacle)));
                outline.dep(big_chest, obstacle);
                outline.dep(obstacle, boss);
            }
        }

        let mut last_locked_door = None;
        for i in 0..config.num_small_keys {
            let locked_door = outline.node(Action::PrependAny(Feature::SmallKeyDoor));
            if let Some(last_locked_door) = last_locked_door {
                outline.dep(locked_door, last_locked_door);
            } else {
                outline.dep(locked_door, big_key);
            }
            if i == config.num_small_keys - 1 {
                let mut last_small_key = None;
                for j in 0..config.num_small_keys {
                    let small_key =
                        outline.node(Action::New(Feature::SmallChest(Treasure::SmallKey)));
                    if let Some(last_small_key) = last_small_key {
                        outline.dep(small_key, last_small_key);
                    } else {
                        outline.dep(small_key, locked_door);
                    }
                    if j == config.num_small_keys - 1 {
                        outline.dep(entrance, small_key);
                    }
                    last_small_key = Some(small_key);
                }
            }
            last_locked_door = Some(locked_door);
        }
        if config.num_small_keys == 0 {
            outline.dep(entrance, big_key);
        }

        let map = outline.node(Action::New(Feature::SmallChest(Treasure::Map)));
        if let Some(weak_wall) =
            outline.index(Action::PrependEach(Feature::Obstacle(Obstacle::WeakWall)))
        {
            let very_weak_wall = outline
                .index(Action::PrependEach(Feature::Obstacle(
                    Obstacle::VeryWeakWall,
                )))
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
    NoChest,
    BigKey,
    BombsCounter,
    Map,
    Compass,
    SmallKey,
}

impl Treasure {
    fn get_obstacles(self) -> &'static [Obstacle] {
        match self {
            Treasure::NoChest => &[],
            Treasure::BigKey => &[],
            Treasure::BombsCounter => &[Obstacle::WeakWall, Obstacle::VeryWeakWall],
            Treasure::Map => &[],
            Treasure::Compass => &[],
            Treasure::SmallKey => &[],
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Feature {
    Boss,
    CulDeSac,
    Fairy,
    Obstacle(Obstacle),
    SmallChest(Treasure),
    HiddenSmallChest(Treasure),
    BigChest(Treasure),
    SmallKeyDoor,
    Entrance,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct HideSmallChests;

impl Transform<Feature> for HideSmallChests {
    fn apply<R: Rng>(&self, rng: &mut R, mut tree: Tree<Feature>) -> Tree<Feature> {
        fn visit<R: Rng>(rng: &mut R, tree: &mut Tree<Feature>, is_below: bool) {
            match tree {
                Tree::Feature(feature @ Feature::SmallChest(_), next) if is_below => {
                    let treasure = if let Feature::SmallChest(treasure) = feature {
                        *treasure
                    } else {
                        unreachable!()
                    };
                    if rng.gen_bool(0.5) {
                        *feature = Feature::HiddenSmallChest(treasure);
                    }
                    visit(rng, next, true);
                }
                Tree::Feature(_, next) => visit(rng, next, is_below),
                Tree::Branch(nodes) => {
                    for node in nodes {
                        visit(rng, node, is_below);
                    }
                }
            }
        }
        visit(rng, &mut tree, false);
        tree
    }
}

pub fn calc_join_weight(
    tree: &Tree<Feature>,
    global_max_depth: usize,
) -> Box<dyn Fn(&Tree<Feature>) -> usize> {
    fn big_key_pred(feature: &Feature) -> bool {
        matches!(
            feature,
            Feature::Boss | Feature::BigChest(_) | Feature::SmallChest(Treasure::BigKey)
        )
    }
    let depth = tree.find_feature_depth(&big_key_pred);
    if depth.is_some() {
        Box::new(move |node| {
            let node_max_depth = node.max_depth();
            let big_key_depth = node
                .find_feature_depth(&big_key_pred)
                .unwrap_or(global_max_depth);
            NODE_DEPTH_WEIGHT * (global_max_depth - node_max_depth)
                + BIG_KEY_DEPTH_WEIGHT * big_key_depth
                + 1
        })
    } else {
        Box::new(move |node| {
            let node_max_depth = node.max_depth();
            let big_key_depth = node
                .find_feature_depth(&big_key_pred)
                .unwrap_or(global_max_depth);
            NODE_DEPTH_WEIGHT * (global_max_depth - node_max_depth)
                + BIG_KEY_DEPTH_WEIGHT * (global_max_depth - big_key_depth)
                + 1
        })
    }
}

pub struct Compacter {
    pub max_heads: usize,
}

impl event_tree::Compacter<Feature> for Compacter {
    fn compact<R>(&self, rng: &mut R, tree: Tree<Feature>) -> Tree<Feature>
    where
        R: Rng,
    {
        match tree {
            Tree::Branch(mut nodes) => {
                while nodes.len() > self.max_heads {
                    let head = nodes.remove(rng.gen_range(0..nodes.len()));
                    let max_depth = nodes.iter().fold(0, |acc: usize, node: &Tree<Feature>| {
                        acc.max(node.max_depth())
                    });
                    let calc_join_weight = calc_join_weight(&head, max_depth);
                    let dist = WeightedIndex::new(nodes.iter().map(calc_join_weight)).unwrap();
                    nodes.get_mut(dist.sample(rng)).unwrap().join(head);
                }
                Tree::Branch(nodes)
            }
            _ => tree,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Lock {
    SmallKey,
    BigKey,
    Full,
}

#[derive(Clone)]
pub struct Room {
    pub entrance: Option<Lock>,
    pub exits: Vec<Room>,
    pub chest: Option<Treasure>,
    pub obstacle: Option<Obstacle>,
    pub far_side_chest: Option<bool>,
}

impl Default for Room {
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

impl Room {
    pub fn show(&self) {
        fn visit(indent: usize, room: &Room) {
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
                "{:indent$}* {}:{}:{}:{}",
                "",
                lock,
                room.chest
                    .as_ref()
                    .map(|v| format!("{:?}", v))
                    .unwrap_or_else(|| "".to_string()),
                side,
                room.obstacle
                    .as_ref()
                    .map(|v| format!("{:?}", v))
                    .unwrap_or_else(|| "".to_string()),
                indent = indent
            );
            for exit in &room.exits {
                visit(indent + 2, exit);
            }
        }
        visit(0, self);
    }
}

impl event_tree::Room for Room {
    fn add_exits<I>(mut self, exits: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        self.exits.extend(exits);
        self
    }
}

impl event_tree::Feature for Feature {
    type Room = Room;

    fn apply(self, mut room: Room) -> Result<Room, (Self, Room)> {
        match self {
            Feature::Boss
                if room.entrance.is_none() && room.chest.is_none() && room.obstacle.is_none() =>
            {
                room.obstacle = Some(Obstacle::Boss);
                room.entrance = Some(Lock::BigKey);
                Ok(room)
            }
            Feature::Obstacle(obstacle) if room.entrance.is_none() && room.obstacle.is_none() => {
                room.obstacle = Some(obstacle);
                if room.chest.is_some() {
                    room.far_side_chest = Some(true);
                }
                Ok(room)
            }
            Feature::SmallChest(treasure) if room.entrance.is_none() && room.chest.is_none() => {
                room.chest = Some(treasure);
                if room.obstacle.is_some() {
                    room.far_side_chest = Some(false);
                }
                Ok(room)
            }
            Feature::HiddenSmallChest(treasure)
                if room.entrance.is_none() && room.chest.is_none() && room.obstacle.is_none() =>
            {
                room.chest = Some(treasure);
                room.obstacle = Some(Obstacle::Puzzle);
                Ok(room)
            }
            Feature::BigChest(treasure)
                if room.entrance.is_none() && room.chest.is_none() && room.obstacle.is_none() =>
            {
                room.chest = Some(treasure);
                room.obstacle = Some(Obstacle::BigChest);
                Ok(room)
            }
            Feature::SmallKeyDoor if room.entrance.is_none() => {
                room.entrance = Some(Lock::SmallKey);
                Ok(room)
            }
            Feature::Entrance
                if room.entrance.is_none() && room.chest.is_none() && room.obstacle.is_none() =>
            {
                room.obstacle = Some(Obstacle::Entrance);
                Ok(room)
            }
            Feature::CulDeSac
                if room.entrance.is_none() && room.chest.is_none() && room.obstacle.is_none() =>
            {
                room.obstacle = Some(Obstacle::CulDeSac);
                room.chest = Some(Treasure::NoChest);
                Ok(room)
            }
            Feature::Fairy
                if room.entrance.is_none() && room.chest.is_none() && room.obstacle.is_none() =>
            {
                room.obstacle = Some(Obstacle::Fairy);
                room.chest = Some(Treasure::NoChest);
                Ok(room)
            }
            _ => Err((self, room)),
        }
    }
}
