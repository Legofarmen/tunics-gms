use crate::core::build::BuildPlan;
use crate::core::build::Index;
use crate::core::feature::FeaturePlan;
use crate::core::feature::Op;
use crate::core::floor::Dir4;
use crate::core::room::RoomExt;
use crate::core::room::Tree as RoomTree;
use rand::Rng;
use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

const NODE_DEPTH_WEIGHT: usize = 1;
const BIG_KEY_DEPTH_WEIGHT: usize = 2;
const MAX_WIDTH: usize = 3;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Feature {
    Door(Door),
    Room(Contents),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Door {
    DungeonEntrance,
    SmallKeyLock,
    BigKeyLock,

    /// A wall that you can bomb through.
    /// Visible on the map but invisible to the eye.
    WeakWall,

    /// A wall that you can bomb through.
    /// Visible both on the map and to the eye.
    VeryWeakWall,

    /// Opens when the parent room is activated.
    ActivationLock,

    /// When the hero enters through it, all doors in the inner room is shut
    /// around the player.
    /// The doors open when the room is activated.
    Trap,
}

impl fmt::Display for Door {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Contents {
    Empty,
    Boss,
    Fairy,
    SmallChest(Treasure),
    BigChest(Item),

    /// Appears when the hero finds a secret.
    SecretChest(Treasure),

    /// An obstacle the hero needs to swim across.
    /// The entrance is on the near side, and all exits on the far side.
    Mote,

    /// An obstacle the hero needs to grapple across.
    /// The entrance is on the near side, and all exits on the far side.
    Chasm,

    /// An obstacle the hero needs great plysical strength to cross.
    /// The entrance is on the near side, and all exits on the far side.
    Rubble,

    /// The room is activated by shoot with an arrow to open.
    ArrowChallenge,

    /// Lets the hero activate the room by use of strength.
    StrengthChallenge,

    /// Lets the hero activate the room by use of fire.
    FireChallenge,

    /// Lets the hero activate the room by succeeding in combat.
    CombatChallenge,
}

impl fmt::Display for Contents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Contents::BigChest(treasure) => write!(f, "#{:?}", treasure),
            Contents::SmallChest(treasure) => write!(f, "+{:?}", treasure),
            Contents::SecretChest(treasure) => write!(f, "?{:?}", treasure),
            contents => write!(f, "{:?}", contents),
        }
    }
}

impl Item {
    pub fn all() -> Vec<Item> {
        vec![
            Item::BombBag,
            Item::Bow,
            Item::Grapple,
            Item::Glove,
            Item::Lantern,
            Item::Flippers,
        ]
    }
}

#[derive(Clone)]
pub struct Config {
    pub items: BTreeSet<Item>,
    pub num_small_keys: usize,
    pub num_traps: usize,
    pub num_fairies: usize,
    pub num_cul_de_sacs: usize,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "items: {:?}, ", self.items)?;
        write!(f, "small keys: {}, ", self.num_small_keys)?;
        write!(f, "traps: {}, ", self.num_traps)?;
        write!(f, "fairies: {}, ", self.num_fairies)?;
        write!(f, "cul-de-sacs: {}", self.num_cul_de_sacs)
    }
}

impl From<Config> for BuildPlan<AugFeature> {
    fn from(config: Config) -> BuildPlan<AugFeature> {
        let mut build_plan = BuildPlan::new();
        let entrance = build_plan.vertex(Op::PrependGrouped(AugFeature::Feature(Feature::Door(
            Door::DungeonEntrance,
        ))));

        for _ in 0..config.num_cul_de_sacs {
            let cul_de_sac =
                build_plan.vertex(Op::New(AugFeature::Feature(Feature::Room(Contents::Empty))));
            build_plan.arc(entrance, cul_de_sac);
        }

        for _ in 0..config.num_fairies {
            let fairy =
                build_plan.vertex(Op::New(AugFeature::Feature(Feature::Room(Contents::Fairy))));
            build_plan.arc(entrance, fairy);
        }

        for _ in 0..config.num_traps {
            let fairy = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Room(
                Contents::CombatChallenge,
            ))));
            build_plan.arc(entrance, fairy);
        }

        let boss = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Room(Contents::Boss))));
        let big_key = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Room(
            Contents::SmallChest(Treasure::BigKey),
        ))));
        build_plan.arc(big_key, boss);

        let hide_chests = build_plan.vertex(Op::PrependEach(AugFeature::HideSmallChests));
        let compass = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Room(
            Contents::SmallChest(Treasure::Compass),
        ))));
        build_plan.arc(hide_chests, boss);
        build_plan.arc(compass, hide_chests);
        build_plan.arc(entrance, compass);

        for item in &config.items {
            let big_chest = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Room(
                Contents::BigChest(*item),
            ))));
            build_plan.arc(big_key, big_chest);
            for obstacle in item.get_obstacles() {
                let obstacle =
                    build_plan.vertex(Op::PrependEach(AugFeature::Feature(obstacle.clone())));
                build_plan.arc(big_chest, obstacle);
                build_plan.arc(obstacle, boss);
            }
        }

        let mut last_locked_door = None;
        for i in 0..config.num_small_keys {
            let locked_door = build_plan.vertex(Op::PrependOne(AugFeature::Feature(
                Feature::Door(Door::SmallKeyLock),
            )));
            if let Some(last_locked_door) = last_locked_door {
                build_plan.arc(locked_door, last_locked_door);
            } else {
                build_plan.arc(locked_door, big_key);
            }
            if i == config.num_small_keys - 1 {
                let mut last_small_key = None;
                for j in 0..config.num_small_keys {
                    let small_key = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Room(
                        Contents::SmallChest(Treasure::SmallKey),
                    ))));
                    if let Some(last_small_key) = last_small_key {
                        build_plan.arc(small_key, last_small_key);
                    } else {
                        build_plan.arc(small_key, locked_door);
                    }
                    if j == config.num_small_keys - 1 {
                        build_plan.arc(entrance, small_key);
                    }
                    last_small_key = Some(small_key);
                }
            }
            last_locked_door = Some(locked_door);
        }
        if config.num_small_keys == 0 {
            build_plan.arc(entrance, big_key);
        }

        let map = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Room(
            Contents::SmallChest(Treasure::Map),
        ))));
        if let Some(weak_wall) = build_plan.index(Op::PrependEach(AugFeature::Feature(
            Feature::Door(Door::WeakWall),
        ))) {
            build_plan.arc(weak_wall, map);
        } else {
            build_plan.arc(entrance, map);
        }
        build_plan.arc(map, boss);

        build_plan
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Treasure {
    SmallKey,
    BigKey,
    Map,
    Compass,
    Item(Item),
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Item {
    BombBag,
    Bow,
    Flippers,
    Glove,
    Grapple,
    Lantern,
}

impl FromStr for Item {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Item, Self::Err> {
        match s {
            "bomb-bag" => Ok(Item::BombBag),
            "bow" => Ok(Item::Bow),
            "flippers" => Ok(Item::Flippers),
            "glove" => Ok(Item::Glove),
            "grapple" => Ok(Item::Grapple),
            "lantern" => Ok(Item::Lantern),
            _ => Err("invalid item"),
        }
    }
}

impl Item {
    fn get_obstacles(self) -> &'static [Feature] {
        match self {
            Item::BombBag => &[Feature::Door(Door::WeakWall)],
            Item::Glove => &[Feature::Room(Contents::Rubble)],
            Item::Grapple => &[Feature::Room(Contents::Chasm)],
            Item::Flippers => &[Feature::Room(Contents::Mote)],
            Item::Bow => &[Feature::Room(Contents::ArrowChallenge)],
            Item::Lantern => &[Feature::Room(Contents::FireChallenge)],
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum AugFeature {
    Feature(Feature),
    HideSmallChests,
}

impl fmt::Debug for AugFeature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AugFeature::Feature(feature) => feature.fmt(f),
            AugFeature::HideSmallChests => write!(f, "< HideSmallChests >"),
        }
    }
}

pub fn lower<R: Rng>(rng: &mut R, feature_plan: FeaturePlan<AugFeature>) -> FeaturePlan<Feature> {
    fn visit<R: Rng>(
        rng: &mut R,
        tree: FeaturePlan<AugFeature>,
        is_below: bool,
    ) -> (FeaturePlan<Feature>, bool) {
        match tree {
            FeaturePlan::Feature(
                AugFeature::Feature(Feature::Room(Contents::SmallChest(treasure))),
                next,
            ) if is_below => {
                let (next, has_map) = visit(rng, *next, true);
                (
                    next.prepended(Feature::Room(Contents::SecretChest(treasure))),
                    has_map || treasure == Treasure::Map,
                )
            }
            FeaturePlan::Feature(AugFeature::Feature(Feature::Door(Door::WeakWall)), next) => {
                let (next, has_map) = visit(rng, *next, true);
                if has_map {
                    (next.prepended(Feature::Door(Door::VeryWeakWall)), has_map)
                } else {
                    (next.prepended(Feature::Door(Door::WeakWall)), has_map)
                }
            }
            FeaturePlan::Feature(AugFeature::Feature(Feature::Room(Contents::Boss)), next) => {
                let (next, has_map) = visit(rng, *next, is_below);
                (
                    next.prepended(Feature::Room(Contents::Boss))
                        .prepended(Feature::Door(Door::BigKeyLock))
                        .prepended(Feature::Room(Contents::Empty)),
                    has_map,
                )
            }
            FeaturePlan::Feature(
                AugFeature::Feature(Feature::Room(Contents::CombatChallenge)),
                next,
            ) => {
                let (next, has_map) = visit(rng, *next, is_below);
                (
                    next.prepended(Feature::Room(Contents::CombatChallenge))
                        .prepended(Feature::Door(Door::Trap)),
                    has_map,
                )
            }
            FeaturePlan::Feature(
                AugFeature::Feature(Feature::Door(Door::DungeonEntrance)),
                next,
            ) => {
                let (next, has_map) = visit(rng, *next, is_below);
                (
                    next.prepended(Feature::Room(Contents::Empty))
                        .prepended(Feature::Door(Door::DungeonEntrance)),
                    has_map,
                )
            }
            FeaturePlan::Feature(AugFeature::Feature(feature), next) => {
                let (next, has_map) = visit(rng, *next, is_below);
                (next.prepended(feature), has_map)
            }
            FeaturePlan::Branch(nodes) => nodes
                .into_iter()
                .map(|node| visit(rng, node, is_below))
                .reduce(|(acc, acc_has_map), (node, has_map)| {
                    (acc.join(node), acc_has_map | has_map)
                })
                .unwrap_or_else(|| (FeaturePlan::default(), false)),
            FeaturePlan::Feature(AugFeature::HideSmallChests, next) => visit(rng, *next, true),
        }
    }
    visit(rng, feature_plan, false).0
}

pub fn get_traversal_selector<R>(
    mut rng: R,
    build_plan: &BuildPlan<AugFeature>,
) -> impl FnMut(&[Index]) -> Index
where
    R: Rng,
{
    use rand::seq::SliceRandom;
    let weights = build_plan.reachable_counts();
    move |open: &[Index]| {
        *open
            .choose_weighted(&mut rng, |index| weights.get(index).unwrap())
            .unwrap()
    }
}

pub fn get_prepend_selector<R>(mut rng: R) -> impl FnMut(&[FeaturePlan<AugFeature>]) -> usize
where
    R: Rng,
{
    move |nodes: &[FeaturePlan<AugFeature>]| rng.gen_range(0..nodes.len())
}

pub fn get_join_selector<'a, R>(
    mut rng: R,
) -> impl FnMut(&[FeaturePlan<AugFeature>]) -> Option<(usize, usize)>
where
    R: 'a + Rng,
{
    use rand::distributions::Distribution;
    use rand::distributions::WeightedIndex;

    move |nodes: &[FeaturePlan<AugFeature>]| {
        if nodes.len() > MAX_WIDTH {
            fn big_key_pred(feature: &AugFeature) -> bool {
                matches!(
                    feature,
                    AugFeature::Feature(
                        Feature::Room(Contents::Boss)
                            | Feature::Room(Contents::BigChest(_))
                            | Feature::Room(Contents::SmallChest(Treasure::BigKey))
                            | Feature::Room(Contents::SecretChest(Treasure::BigKey))
                    )
                )
            }
            let i = rng.gen_range(0..nodes.len());
            let depth = nodes[i].find_feature_depth(&big_key_pred);
            let global_max_depth = nodes
                .iter()
                .enumerate()
                .filter_map(|(j, n)| if j == i { None } else { Some(n) })
                .fold(0, |acc: usize, node: &FeaturePlan<_>| {
                    acc.max(node.max_depth())
                });
            let calc_join_weight: Box<dyn Fn(_) -> _> = if depth.is_some() {
                Box::new(move |node: &FeaturePlan<AugFeature>| {
                    let node_max_depth = node.max_depth();
                    let big_key_depth = node
                        .find_feature_depth(&big_key_pred)
                        .unwrap_or(global_max_depth);
                    NODE_DEPTH_WEIGHT * (global_max_depth - node_max_depth)
                        + BIG_KEY_DEPTH_WEIGHT * big_key_depth
                        + 1
                })
            } else {
                Box::new(move |node: &FeaturePlan<AugFeature>| {
                    let node_max_depth = node.max_depth();
                    let big_key_depth = node
                        .find_feature_depth(&big_key_pred)
                        .unwrap_or(global_max_depth);
                    NODE_DEPTH_WEIGHT * (global_max_depth - node_max_depth)
                        + BIG_KEY_DEPTH_WEIGHT * (global_max_depth - big_key_depth)
                        + 1
                })
            };

            let dist = WeightedIndex::new(nodes.iter().enumerate().map(|(k, node)| {
                if k == i {
                    0
                } else {
                    calc_join_weight(node)
                }
            }))
            .unwrap();
            let j = dist.sample(&mut rng);
            Some((i, j))
        } else {
            None
        }
    }
}

impl RoomExt for RoomTree<Door, Contents> {
    type Feature = Feature;

    fn is_boundary(&self) -> bool {
        if self.entrance.is_some() {
            return true;
        }
        match self.contents {
            Some(Contents::Mote)
            | Some(Contents::Chasm)
            | Some(Contents::Rubble)
            | Some(Contents::ArrowChallenge)
            | Some(Contents::StrengthChallenge)
            | Some(Contents::FireChallenge)
            | Some(Contents::CombatChallenge) => true,
            _ => false,
        }
    }

    fn is_dir_compatible(&self, dir: Dir4) -> bool {
        !(dir != Dir4::North && self.entrance == Some(Door::BigKeyLock))
    }

    fn add_feature(mut self, feature: Feature) -> Result<Self, (Feature, Self)> {
        if self.entrance.is_some() {
            return Err((feature, self));
        }
        match feature {
            Feature::Door(door) => {
                self.entrance = Some(door);
                Ok(self)
            }
            Feature::Room(
                contents @ Contents::BigChest(_)
                | contents @ Contents::Boss
                | contents @ Contents::Chasm
                | contents @ Contents::CombatChallenge
                | contents @ Contents::Empty
                | contents @ Contents::Fairy
                | contents @ Contents::Mote
                | contents @ Contents::Rubble
                | contents @ Contents::SecretChest(_)
                | contents @ Contents::SmallChest(_),
            ) if self.contents.is_none() => {
                self.contents = Some(contents);
                Ok(self)
            }
            Feature::Room(
                contents @ Contents::ArrowChallenge
                | contents @ Contents::FireChallenge
                | contents @ Contents::StrengthChallenge,
            ) if self.entrance.is_none() => {
                self.entrance = Some(Door::ActivationLock);
                Ok(RoomTree {
                    entrance: None,
                    contents: Some(contents),
                    exits: [self].into(),
                })
            }
            _ => Err((feature, self)),
        }
    }
}
