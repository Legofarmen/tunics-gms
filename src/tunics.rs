use crate::feature_tree;
use crate::feature_tree::Action;
use crate::feature_tree::Transform;
use crate::requirements;
use rand::distributions::weighted::WeightedIndex;
use rand::distributions::Distribution;
use rand::Rng;
use std::collections::HashSet;

type FeatureTree = feature_tree::FeatureTree<Feature>;
type Requirements = requirements::Requirements<Feature, HideSmallChests>;

const NODE_DEPTH_WEIGHT: usize = 1;
const BIG_KEY_DEPTH_WEIGHT: usize = 2;

pub struct Config {
    pub num_small_keys: usize,
    pub num_fairies: usize,
    pub num_cul_de_sacs: usize,
    pub treasures: HashSet<Treasure>,
}

impl From<Config> for Requirements {
    fn from(config: Config) -> Requirements {
        let mut requirements = Requirements::new();
        let entrance = requirements.node(Action::PrependGrouped(Feature::Entrance));

        for _ in 0..config.num_cul_de_sacs {
            let cul_de_sac = requirements.node(Action::New(Feature::CulDeSac));
            requirements.dep(entrance, cul_de_sac);
        }

        for _ in 0..config.num_fairies {
            let fairy = requirements.node(Action::New(Feature::Fairy));
            requirements.dep(entrance, fairy);
        }

        let boss = requirements.node(Action::New(Feature::Boss));
        let big_key = requirements.node(Action::New(Feature::SmallChest(Treasure::BigKey)));
        requirements.dep(big_key, boss);

        let hide_chests = requirements.node(Action::TransformEach(HideSmallChests));
        let compass = requirements.node(Action::New(Feature::SmallChest(Treasure::Compass)));
        requirements.dep(hide_chests, boss);
        requirements.dep(compass, hide_chests);
        requirements.dep(entrance, compass);

        for treasure in &config.treasures {
            let big_chest = requirements.node(Action::New(Feature::BigChest(*treasure)));
            requirements.dep(big_key, big_chest);
            for obstacle in treasure.get_obstacles() {
                let obstacle = requirements.node(Action::PrependEach(Feature::Obstacle(*obstacle)));
                requirements.dep(big_chest, obstacle);
                requirements.dep(obstacle, boss);
            }
        }

        let mut last_locked_door = None;
        for i in 0..config.num_small_keys {
            let locked_door = requirements.node(Action::PrependAny(Feature::SmallKeyDoor));
            if let Some(last_locked_door) = last_locked_door {
                requirements.dep(locked_door, last_locked_door);
            } else {
                requirements.dep(locked_door, big_key);
            }
            if i == config.num_small_keys - 1 {
                let mut last_small_key = None;
                for j in 0..config.num_small_keys {
                    let small_key =
                        requirements.node(Action::New(Feature::SmallChest(Treasure::SmallKey)));
                    if let Some(last_small_key) = last_small_key {
                        requirements.dep(small_key, last_small_key);
                    } else {
                        requirements.dep(small_key, locked_door);
                    }
                    if j == config.num_small_keys - 1 {
                        requirements.dep(entrance, small_key);
                    }
                    last_small_key = Some(small_key);
                }
            }
            last_locked_door = Some(locked_door);
        }
        if config.num_small_keys == 0 {
            requirements.dep(entrance, big_key);
        }

        let map = requirements.node(Action::New(Feature::SmallChest(Treasure::Map)));
        if let Some(weak_wall) =
            requirements.index(Action::PrependEach(Feature::Obstacle(Obstacle::WeakWall)))
        {
            let very_weak_wall = requirements
                .index(Action::PrependEach(Feature::Obstacle(
                    Obstacle::VeryWeakWall,
                )))
                .unwrap();
            requirements.dep(very_weak_wall, weak_wall);
            requirements.dep(map, very_weak_wall);
        } else {
            requirements.dep(map, boss);
        }
        requirements.dep(entrance, map);

        requirements
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HideSmallChests;

impl Transform<Feature> for HideSmallChests {
    fn apply<R: Rng>(&self, rng: &mut R, tree: FeatureTree) -> FeatureTree {
        fn visit<R: Rng>(rng: &mut R, tree: FeatureTree, is_below: bool) -> FeatureTree {
            match tree {
                FeatureTree::Feature(Feature::SmallChest(treasure), next) if is_below => {
                    let next = visit(rng, *next, true);
                    if rng.gen_bool(0.5) {
                        next.prepended(Feature::HiddenSmallChest(treasure))
                    } else {
                        next.prepended(Feature::SmallChest(treasure))
                    }
                }
                FeatureTree::Feature(feature, next) => {
                    visit(rng, *next, is_below).prepended(feature)
                }
                FeatureTree::Branch(nodes) => FeatureTree::Branch(
                    nodes
                        .into_iter()
                        .map(|node| visit(rng, node, is_below))
                        .collect(),
                ),
            }
        }
        visit(rng, tree, false)
    }
}

pub fn calc_join_weight(
    tree: &FeatureTree,
    global_max_depth: usize,
) -> Box<dyn Fn(&FeatureTree) -> usize> {
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

pub fn compact<R>(rng: &mut R, max_width: usize, tree: FeatureTree) -> FeatureTree
where
    R: Rng,
{
    match tree {
        FeatureTree::Branch(mut nodes) => {
            while nodes.len() > max_width {
                let head = nodes.remove(rng.gen_range(0..nodes.len()));
                let max_depth = nodes.iter().fold(0, |acc: usize, node: &FeatureTree| {
                    acc.max(node.max_depth())
                });
                let calc_join_weight = calc_join_weight(&head, max_depth);
                let dist = WeightedIndex::new(nodes.iter().map(calc_join_weight)).unwrap();
                nodes.get_mut(dist.sample(rng)).unwrap().join(head);
            }
            FeatureTree::Branch(nodes)
        }
        _ => tree,
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

impl feature_tree::Room for Room {
    fn add_exits<I>(mut self, exits: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        self.exits.extend(exits);
        self
    }
}

impl feature_tree::Feature for Feature {
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
