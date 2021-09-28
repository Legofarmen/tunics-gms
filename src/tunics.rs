use crate::core::build::BuildPlan;
use crate::core::build::Index;
use crate::core::feature;
use crate::core::feature::FeaturePlan;
use crate::core::feature::Op;
use rand::Rng;
use std::collections::HashSet;

const NODE_DEPTH_WEIGHT: usize = 1;
const BIG_KEY_DEPTH_WEIGHT: usize = 2;
const MAX_WIDTH: usize = 3;

type Feature = feature::Feature<Interior, Obstacle>;

pub struct Config {
    pub num_small_keys: usize,
    pub num_fairies: usize,
    pub num_cul_de_sacs: usize,
    pub treasures: HashSet<Treasure>,
}

impl From<Config> for BuildPlan<AugFeature> {
    fn from(config: Config) -> BuildPlan<AugFeature> {
        let mut build_plan = BuildPlan::new();
        let entrance = build_plan.vertex(Op::PrependGrouped(AugFeature::Feature(
            Feature::Interior(Interior::Entrance),
        )));

        for _ in 0..config.num_cul_de_sacs {
            let cul_de_sac = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Interior(
                Interior::CulDeSac,
            ))));
            build_plan.arc(entrance, cul_de_sac);
        }

        for _ in 0..config.num_fairies {
            let fairy = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Interior(
                Interior::Fairy,
            ))));
            build_plan.arc(entrance, fairy);
        }

        let boss = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Interior(
            Interior::Boss,
        ))));
        let big_key = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Interior(
            Interior::SmallChest(Treasure::BigKey),
        ))));
        build_plan.arc(big_key, boss);

        let hide_chests = build_plan.vertex(Op::PrependEach(AugFeature::HideSmallChests));
        let compass = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Interior(
            Interior::SmallChest(Treasure::Compass),
        ))));
        build_plan.arc(hide_chests, boss);
        build_plan.arc(compass, hide_chests);
        build_plan.arc(entrance, compass);

        for treasure in &config.treasures {
            let big_chest = build_plan.vertex(Op::New(AugFeature::Feature(Feature::Interior(
                Interior::BigChest(*treasure),
            ))));
            build_plan.arc(big_key, big_chest);
            for obstacle in treasure.get_obstacles() {
                let obstacle = build_plan.vertex(Op::PrependEach(AugFeature::Feature(
                    Feature::Obstacle(Obstacle {
                        kind: *obstacle,
                        treasure: None,
                    }),
                )));
                build_plan.arc(big_chest, obstacle);
                build_plan.arc(obstacle, boss);
            }
        }

        let mut last_locked_door = None;
        for i in 0..config.num_small_keys {
            let locked_door = build_plan.vertex(Op::PrependOne(AugFeature::Feature(
                Feature::Obstacle(Obstacle {
                    kind: ObstacleKind::Door(Door::SmallKey),
                    treasure: None,
                }),
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
                        Feature::Interior(Interior::SmallChest(Treasure::SmallKey)),
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
            Interior::SmallChest(Treasure::Map),
        ))));
        if let Some(weak_wall) = build_plan.index(Op::PrependEach(AugFeature::Feature(
            Feature::Obstacle(Obstacle {
                kind: ObstacleKind::Door(Door::WeakWall),
                treasure: None,
            }),
        ))) {
            let very_weak_wall = build_plan
                .index(Op::PrependEach(AugFeature::Feature(Feature::Obstacle(
                    Obstacle {
                        kind: ObstacleKind::Door(Door::VeryWeakWall),
                        treasure: None,
                    },
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
pub struct Obstacle {
    kind: ObstacleKind,
    treasure: Option<Treasure>,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum ObstacleKind {
    Puzzle,
    Door(Door),
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Treasure {
    NoChest,
    BigKey,
    BombBag,
    Map,
    Compass,
    SmallKey,
}

impl Treasure {
    fn get_obstacles(self) -> &'static [ObstacleKind] {
        match self {
            Treasure::NoChest => &[],
            Treasure::BigKey => &[],
            Treasure::BombBag => &[
                ObstacleKind::Door(Door::WeakWall),
                ObstacleKind::Door(Door::VeryWeakWall),
            ],
            Treasure::Map => &[],
            Treasure::Compass => &[],
            Treasure::SmallKey => &[],
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
    BigChest(Treasure),
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
                AugFeature::Feature(Feature::Interior(Interior::SmallChest(treasure))),
                next,
            ) if is_below => {
                let next = visit(rng, *next, true);
                if rng.gen_bool(0.5) {
                    next.prepended(Feature::Obstacle(Obstacle {
                        kind: ObstacleKind::Puzzle,
                        treasure: Some(treasure),
                    }))
                } else {
                    next.prepended(Feature::Interior(Interior::SmallChest(treasure)))
                }
            }
            FeaturePlan::Feature(AugFeature::Feature(Feature::Interior(Interior::Boss)), next) => {
                let next = visit(rng, *next, true);
                next.prepended(Feature::Interior(Interior::Boss))
                    .prepended(Feature::Obstacle(Obstacle {
                        kind: ObstacleKind::Door(Door::BigKey),
                        treasure: None,
                    }))
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

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Door {
    SmallKey,
    BigKey,
    WeakWall,
    VeryWeakWall,
}

#[derive(Clone)]
pub struct Room {
    pub entrance: Option<Door>,
    pub exits: Vec<Room>,
    pub chest: Option<Treasure>,
    pub obstacle: Option<RoomObstacle>,
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
            eprintln!(
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

#[derive(Copy, Clone, Debug)]
pub enum RoomObstacle {
    Obstacle(ObstacleKind),
    Interior(Interior),
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
            Feature::Interior(Interior::SmallChest(treasure))
                if self.entrance.is_none() && self.chest.is_none() =>
            {
                self.chest = Some(treasure);
                if self.obstacle.is_some() {
                    self.far_side_chest = Some(false);
                }
                Ok(self)
            }
            Feature::Obstacle(Obstacle {
                kind: ObstacleKind::Door(Door::SmallKey),
                ..
            }) if self.entrance.is_none() => {
                self.entrance = Some(Door::SmallKey);
                Ok(self)
            }
            Feature::Obstacle(Obstacle {
                kind: ObstacleKind::Door(door),
                treasure: None,
            }) if self.entrance.is_none() => {
                self.entrance = Some(door);
                Ok(self)
            }
            Feature::Obstacle(Obstacle {
                kind: ObstacleKind::Puzzle,
                treasure,
            }) if self.entrance.is_none() && self.obstacle.is_none() && self.chest.is_none() => {
                self.obstacle = Some(RoomObstacle::Obstacle(ObstacleKind::Puzzle));
                self.chest = treasure;
                if self.chest.is_some() {
                    self.far_side_chest = Some(true);
                }
                Ok(self)
            }
            Feature::Interior(interior)
                if self.entrance.is_none() && self.chest.is_none() && self.obstacle.is_none() =>
            {
                self.obstacle = Some(RoomObstacle::Interior(interior));
                Ok(self)
            }
            _ => Err((feature, self)),
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
                        Feature::Interior(Interior::Boss)
                            | Feature::Interior(Interior::BigChest(_))
                            | Feature::Interior(Interior::SmallChest(Treasure::BigKey))
                            | Feature::Obstacle(Obstacle {
                                treasure: Some(Treasure::BigKey),
                                ..
                            })
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
