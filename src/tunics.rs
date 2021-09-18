use crate::core::build;
use crate::core::feature;
use crate::core::feature::Command;
use crate::core::feature::Transform;
use rand::distributions::weighted::WeightedIndex;
use rand::distributions::Distribution;
use rand::Rng;
use std::collections::HashSet;

type FeaturePlan = feature::FeaturePlan<Feature>;
type BuildPlan = build::BuildPlan<Feature, HideSmallChests>;

const NODE_DEPTH_WEIGHT: usize = 1;
const BIG_KEY_DEPTH_WEIGHT: usize = 2;

pub struct Config {
    pub num_small_keys: usize,
    pub num_fairies: usize,
    pub num_cul_de_sacs: usize,
    pub treasures: HashSet<Treasure>,
}

impl From<Config> for BuildPlan {
    fn from(config: Config) -> BuildPlan {
        let mut build_plan = BuildPlan::new();
        let entrance = build_plan.node(Command::PrependGrouped(Feature::Entrance));

        for _ in 0..config.num_cul_de_sacs {
            let cul_de_sac = build_plan.node(Command::New(Feature::CulDeSac));
            build_plan.dep(entrance, cul_de_sac);
        }

        for _ in 0..config.num_fairies {
            let fairy = build_plan.node(Command::New(Feature::Fairy));
            build_plan.dep(entrance, fairy);
        }

        let boss = build_plan.node(Command::New(Feature::Boss));
        let big_key = build_plan.node(Command::New(Feature::SmallChest(Treasure::BigKey)));
        build_plan.dep(big_key, boss);

        let hide_chests = build_plan.node(Command::TransformEach(HideSmallChests));
        let compass = build_plan.node(Command::New(Feature::SmallChest(Treasure::Compass)));
        build_plan.dep(hide_chests, boss);
        build_plan.dep(compass, hide_chests);
        build_plan.dep(entrance, compass);

        for treasure in &config.treasures {
            let big_chest = build_plan.node(Command::New(Feature::BigChest(*treasure)));
            build_plan.dep(big_key, big_chest);
            for obstacle in treasure.get_obstacles() {
                let obstacle = build_plan.node(Command::PrependEach(Feature::Obstacle(*obstacle)));
                build_plan.dep(big_chest, obstacle);
                build_plan.dep(obstacle, boss);
            }
        }

        let mut last_locked_door = None;
        for i in 0..config.num_small_keys {
            let locked_door = build_plan.node(Command::PrependAny(Feature::SmallKeyDoor));
            if let Some(last_locked_door) = last_locked_door {
                build_plan.dep(locked_door, last_locked_door);
            } else {
                build_plan.dep(locked_door, big_key);
            }
            if i == config.num_small_keys - 1 {
                let mut last_small_key = None;
                for j in 0..config.num_small_keys {
                    let small_key =
                        build_plan.node(Command::New(Feature::SmallChest(Treasure::SmallKey)));
                    if let Some(last_small_key) = last_small_key {
                        build_plan.dep(small_key, last_small_key);
                    } else {
                        build_plan.dep(small_key, locked_door);
                    }
                    if j == config.num_small_keys - 1 {
                        build_plan.dep(entrance, small_key);
                    }
                    last_small_key = Some(small_key);
                }
            }
            last_locked_door = Some(locked_door);
        }
        if config.num_small_keys == 0 {
            build_plan.dep(entrance, big_key);
        }

        let map = build_plan.node(Command::New(Feature::SmallChest(Treasure::Map)));
        if let Some(weak_wall) =
            build_plan.index(Command::PrependEach(Feature::Obstacle(Obstacle::WeakWall)))
        {
            let very_weak_wall = build_plan
                .index(Command::PrependEach(Feature::Obstacle(
                    Obstacle::VeryWeakWall,
                )))
                .unwrap();
            build_plan.dep(very_weak_wall, weak_wall);
            build_plan.dep(map, very_weak_wall);
        } else {
            build_plan.dep(map, boss);
        }
        build_plan.dep(entrance, map);

        build_plan
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
    fn apply<R: Rng>(&self, rng: &mut R, tree: FeaturePlan) -> FeaturePlan {
        fn visit<R: Rng>(rng: &mut R, tree: FeaturePlan, is_below: bool) -> FeaturePlan {
            match tree {
                FeaturePlan::Feature(Feature::SmallChest(treasure), next) if is_below => {
                    let next = visit(rng, *next, true);
                    if rng.gen_bool(0.5) {
                        next.prepended(Feature::HiddenSmallChest(treasure))
                    } else {
                        next.prepended(Feature::SmallChest(treasure))
                    }
                }
                FeaturePlan::Feature(feature, next) => {
                    visit(rng, *next, is_below).prepended(feature)
                }
                FeaturePlan::Branch(nodes) => FeaturePlan::Branch(
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
    tree: &FeaturePlan,
    global_max_depth: usize,
) -> Box<dyn Fn(&FeaturePlan) -> usize> {
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

pub fn compact<R>(rng: &mut R, max_width: usize, tree: FeaturePlan) -> FeaturePlan
where
    R: Rng,
{
    match tree {
        FeaturePlan::Branch(mut nodes) => {
            while nodes.len() > max_width {
                let head = nodes.remove(rng.gen_range(0..nodes.len()));
                let max_depth = nodes.iter().fold(0, |acc: usize, node: &FeaturePlan| {
                    acc.max(node.max_depth())
                });
                let calc_join_weight = calc_join_weight(&head, max_depth);
                let dist = WeightedIndex::new(nodes.iter().map(calc_join_weight)).unwrap();
                nodes.get_mut(dist.sample(rng)).unwrap().join(head);
            }
            FeaturePlan::Branch(nodes)
        }
        _ => tree,
    }
}

#[derive(Clone, Debug)]
pub enum Lock {
    SmallKey,
    BigKey,
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

impl feature::Room for Room {
    fn add_exits<I>(mut self, exits: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        self.exits.extend(exits);
        self
    }
}

impl feature::Feature for Feature {
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
