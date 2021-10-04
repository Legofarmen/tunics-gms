use self::room::Contents;
use crate::core::build::BuildPlan;
use crate::core::build::Index;
use crate::core::feature::FeaturePlan;
use crate::core::feature::Op;
use rand::Rng;
use std::collections::HashSet;
use std::str::FromStr;

const NODE_DEPTH_WEIGHT: usize = 1;
const BIG_KEY_DEPTH_WEIGHT: usize = 2;
const MAX_WIDTH: usize = 3;

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Feature {
    Entrance,
    Interior(Contents),
    Obstacle(Obstacle),
}

pub fn gen_treasure_set<R: Rng>(rng: &mut R, n: usize) -> HashSet<Item> {
    let mut all_treasures = vec![
        Item::BombBag,
        Item::Bow,
        Item::Grapple,
        Item::Glove,
        Item::Lantern,
        Item::Flippers,
    ];
    let mut treasures = HashSet::new();
    for _ in 0..n {
        treasures.insert(all_treasures.swap_remove(rng.gen_range(0..all_treasures.len())));
    }
    treasures
}

pub struct Config {
    pub num_small_keys: usize,
    pub num_fairies: usize,
    pub num_cul_de_sacs: usize,
    pub items: HashSet<Item>,
}

impl From<Config> for BuildPlan<AugFeature> {
    fn from(config: Config) -> BuildPlan<AugFeature> {
        let mut build_plan = BuildPlan::new();
        let entrance =
            build_plan.vertex(Op::PrependGrouped(AugFeature::Feature(Feature::Entrance)));

        for _ in 0..config.num_cul_de_sacs {
            let cul_de_sac = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Interior(
                Contents::Empty,
            ))));
            build_plan.arc(entrance, cul_de_sac);
        }

        for _ in 0..config.num_fairies {
            let fairy = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Interior(
                Contents::Fairy,
            ))));
            build_plan.arc(entrance, fairy);
        }

        let boss = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Interior(
            Contents::Boss,
        ))));
        let big_key = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Interior(
            Contents::SmallChest(Treasure::BigKey),
        ))));
        build_plan.arc(big_key, boss);

        let hide_chests = build_plan.vertex(Op::PrependEach(AugFeature::HideSmallChests));
        let compass = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Interior(
            Contents::SmallChest(Treasure::Compass),
        ))));
        build_plan.arc(hide_chests, boss);
        build_plan.arc(compass, hide_chests);
        build_plan.arc(entrance, compass);

        for item in &config.items {
            let big_chest = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Interior(
                Contents::BigChest(*item),
            ))));
            build_plan.arc(big_key, big_chest);
            for obstacle in item.get_obstacles() {
                let obstacle = build_plan.vertex(Op::PrependEach(AugFeature::Feature(
                    Feature::Obstacle(*obstacle),
                )));
                build_plan.arc(big_chest, obstacle);
                build_plan.arc(obstacle, boss);
            }
        }

        let mut last_locked_door = None;
        for i in 0..config.num_small_keys {
            let locked_door = build_plan.vertex(Op::PrependOne(AugFeature::Feature(
                Feature::Obstacle(Obstacle::Door(Door::SmallKeyLock)),
            )));
            if let Some(last_locked_door) = last_locked_door {
                build_plan.arc(locked_door, last_locked_door);
            } else {
                build_plan.arc(locked_door, big_key);
            }
            if i == config.num_small_keys - 1 {
                let mut last_small_key = None;
                for j in 0..config.num_small_keys {
                    let small_key = build_plan.vertex(Op::New(AugFeature::Feature(
                        Feature::Interior(Contents::SmallChest(Treasure::SmallKey)),
                    )));
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

        let map = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Interior(
            Contents::SmallChest(Treasure::Map),
        ))));
        if let Some(weak_wall) = build_plan.index(Op::PrependEach(AugFeature::Feature(
            Feature::Obstacle(Obstacle::Door(Door::WeakWall)),
        ))) {
            let very_weak_wall = build_plan
                .index(Op::PrependEach(AugFeature::Feature(Feature::Obstacle(
                    Obstacle::Door(Door::VeryWeakWall),
                ))))
                .unwrap();
            build_plan.arc(very_weak_wall, weak_wall);
            build_plan.arc(map, very_weak_wall);
        } else {
            build_plan.arc(map, boss);
        }
        build_plan.arc(entrance, map);

        build_plan
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Obstacle {
    Chasm,
    Mote,
    Rubble,
    ArrowChallenge,
    FireChallenge,
    Door(Door),
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Treasure {
    SmallKey,
    BigKey,
    Map,
    Compass,
    Item(Item),
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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
    fn get_obstacles(self) -> &'static [Obstacle] {
        match self {
            Item::BombBag => &[
                Obstacle::Door(Door::WeakWall),
                Obstacle::Door(Door::VeryWeakWall),
            ],
            Item::Glove => &[Obstacle::Rubble],
            Item::Grapple => &[Obstacle::Chasm],
            Item::Flippers => &[Obstacle::Mote],
            Item::Bow => &[Obstacle::ArrowChallenge],
            Item::Lantern => &[Obstacle::FireChallenge],
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Interior {
    Boss,
    CulDeSac,
    Fairy,
    Entrance,
    SmallChest(Treasure),
    BigChest(Item),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AugFeature {
    Feature(Feature),
    HideSmallChests,
}

pub fn lower<R: Rng>(rng: &mut R, feature_plan: FeaturePlan<AugFeature>) -> FeaturePlan<Feature> {
    fn visit<R: Rng>(
        rng: &mut R,
        tree: FeaturePlan<AugFeature>,
        is_below: bool,
    ) -> FeaturePlan<Feature> {
        match tree {
            FeaturePlan::Feature(
                AugFeature::Feature(Feature::Interior(Contents::SmallChest(treasure))),
                next,
            ) if is_below => {
                let next = visit(rng, *next, true);
                //if rng.gen_bool(0.5) {
                next.prepended(Feature::Interior(Contents::SecretChest(treasure)))
                //} else {
                //next.prepended(Feature::Interior(Interior::SmallChest(treasure)))
                //}
            }
            FeaturePlan::Feature(AugFeature::Feature(Feature::Interior(Contents::Boss)), next) => {
                let next = visit(rng, *next, true);
                next.prepended(Feature::Interior(Contents::Boss))
                    .prepended(Feature::Obstacle(Obstacle::Door(Door::BigKeyLock)))
            }
            FeaturePlan::Feature(AugFeature::Feature(feature), next) => {
                visit(rng, *next, is_below).prepended(feature)
            }
            FeaturePlan::Branch(nodes) => nodes
                .into_iter()
                .map(|node| visit(rng, node, is_below))
                .reduce(|acc, node| acc.join(node))
                .unwrap_or_else(|| FeaturePlan::default()),
            FeaturePlan::Feature(AugFeature::HideSmallChests, next) => visit(rng, *next, true),
        }
    }
    visit(rng, feature_plan, false)
}

pub mod room {
    use super::Door;
    use super::Feature;
    use super::Item;
    use super::Obstacle;
    use super::Treasure;
    use crate::core::feature;

    pub struct Room {
        pub entrance: Option<Door>,
        pub contents: Option<Contents>,
        pub exits: Vec<Room>,
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

    impl Default for Room {
        fn default() -> Self {
            Room {
                entrance: None,
                contents: None,
                exits: vec![],
            }
        }
    }

    impl Room {
        pub fn is_simple(&self) -> bool {
            use Contents::*;
            self.contents
                .as_ref()
                .map(|contents| {
                    matches!(
                        contents,
                        Empty | Boss | Fairy | SmallChest(_) | BigChest(_) | SecretChest(_)
                    )
                })
                .unwrap_or(false)
        }

        pub fn weight(&self) -> usize {
            self.exits
                .iter()
                .map(Room::weight)
                .fold(1, |acc, weight| acc + weight)
        }

        pub fn show(&self) {
            fn visit(room: &Room, parent: usize, id: usize) -> usize {
                let mut next = id + 1;
                for child in &room.exits {
                    next = visit(child, id, next);
                }
                let door = match &room.entrance {
                    None => "".to_string(),
                    Some(door) => format!("{:?}", door),
                };
                println!("room{} [label=\"{:?}\"];", id, room.contents);
                println!("room{} -- room{} [label=\"{}\"];", parent, id, door);
                next
            }
            println!("graph {{");
            visit(self, 0, 0);
            println!("}}");
        }
    }

    impl feature::Room for Room {
        type Feature = Feature;

        fn add_exits<I>(mut self, exits: I) -> Self
        where
            I: IntoIterator<Item = Self>,
        {
            self.exits.extend(exits);
            self
        }

        fn add_feature(mut self, feature: Feature) -> Result<Self, (Feature, Self)> {
            match feature {
                Feature::Interior(Contents::SmallChest(treasure))
                    if self.entrance.is_none() && self.contents.is_none() =>
                {
                    self.contents = Some(Contents::SmallChest(treasure));
                    Ok(self)
                }
                Feature::Obstacle(Obstacle::Door(Door::SmallKeyLock))
                    if self.entrance.is_none() =>
                {
                    self.entrance = Some(Door::SmallKeyLock);
                    Ok(self)
                }
                Feature::Obstacle(Obstacle::Door(door)) if self.entrance.is_none() => {
                    self.entrance = Some(door.into());
                    Ok(self)
                }
                Feature::Interior(Contents::SecretChest(treasure))
                    if self.entrance.is_none() && self.contents.is_none() =>
                {
                    self.contents = Some(Contents::SecretChest(treasure.into()));
                    Ok(self)
                }
                Feature::Obstacle(Obstacle::Chasm)
                    if self.entrance.is_none() && self.contents.is_none() =>
                {
                    self.contents = Some(Contents::Chasm);
                    Ok(self)
                }
                Feature::Obstacle(Obstacle::Mote)
                    if self.entrance.is_none() && self.contents.is_none() =>
                {
                    self.contents = Some(Contents::Mote);
                    Ok(self)
                }
                Feature::Obstacle(Obstacle::Rubble)
                    if self.entrance.is_none() && self.contents.is_none() =>
                {
                    self.contents = Some(Contents::Rubble);
                    Ok(self)
                }
                Feature::Obstacle(Obstacle::ArrowChallenge) if self.entrance.is_none() => {
                    self.entrance = Some(Door::ActivationLock);
                    Ok(Room {
                        entrance: None,
                        contents: Some(Contents::ArrowChallenge),
                        exits: vec![self],
                    })
                }
                Feature::Obstacle(Obstacle::FireChallenge) if self.entrance.is_none() => {
                    self.entrance = Some(Door::ActivationLock);
                    Ok(Room {
                        entrance: None,
                        contents: Some(Contents::FireChallenge),
                        exits: vec![self],
                    })
                }
                Feature::Entrance if self.entrance.is_none() && self.contents.is_none() => {
                    self.entrance = Some(Door::DungeonEntrance);
                    Ok(self)
                }
                Feature::Interior(interior)
                    if self.entrance.is_none() && self.contents.is_none() =>
                {
                    self.contents = Some(interior.into());
                    Ok(self)
                }
                _ => Err((feature, self)),
            }
        }
    }
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
                        Feature::Interior(Contents::Boss)
                            | Feature::Interior(Contents::BigChest(_))
                            | Feature::Interior(Contents::SmallChest(Treasure::BigKey))
                            | Feature::Interior(Contents::SecretChest(Treasure::BigKey))
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
