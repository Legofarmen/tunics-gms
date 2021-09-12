use crate::event_tree::Action;
use crate::event_tree::Tree;
use crate::outline::Outline;
use rand::Rng;
use std::collections::HashSet;

pub struct OutlineConf {
    pub num_small_keys: usize,
    pub num_fairies: usize,
    pub num_cul_de_sacs: usize,
    pub treasures: HashSet<Treasure>,
}

impl OutlineConf {
    pub fn into_outline(self) -> Outline<EventTreeAction> {
        use EventTreeAction::*;

        let mut outline = Outline::new();
        let entrance = outline.node(PrependGrouped(Event::Entrance));

        for _ in 0..self.num_cul_de_sacs {
            let cul_de_sac = outline.node(AddEvent(Event::CulDeSac));
            outline.dep(entrance, cul_de_sac);
        }

        for _ in 0..self.num_fairies {
            let fairy = outline.node(AddEvent(Event::Fairy));
            outline.dep(entrance, fairy);
        }

        let boss = outline.node(AddEvent(Event::Boss));
        let big_key = outline.node(AddEvent(Event::Chest(Treasure::BigKey)));
        outline.dep(big_key, boss);

        let hide_chests = outline.node(HideChests);
        let compass = outline.node(AddEvent(Event::Chest(Treasure::Compass)));
        outline.dep(hide_chests, boss);
        outline.dep(compass, hide_chests);
        outline.dep(entrance, compass);

        for treasure in &self.treasures {
            let big_chest = outline.node(AddEvent(Event::BigChest(*treasure)));
            outline.dep(big_key, big_chest);
            for obstacle in treasure.get_obstacles() {
                let obstacle = outline.node(PrependEach(Event::Obstacle(*obstacle)));
                outline.dep(big_chest, obstacle);
                outline.dep(obstacle, boss);
            }
        }

        let mut last_locked_door = None;
        for i in 0..self.num_small_keys {
            let locked_door = outline.node(PrependAny(Event::LockedDoor));
            if let Some(last_locked_door) = last_locked_door {
                outline.dep(locked_door, last_locked_door);
            } else {
                outline.dep(locked_door, big_key);
            }
            if i == self.num_small_keys - 1 {
                let mut last_small_key = None;
                for j in 0..self.num_small_keys {
                    let small_key = outline.node(AddEvent(Event::Chest(Treasure::SmallKey)));
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

        let map = outline.node(AddEvent(Event::Chest(Treasure::Map)));
        if let Some(weak_wall) = outline.index(PrependEach(Event::Obstacle(Obstacle::WeakWall))) {
            let very_weak_wall = outline
                .index(PrependEach(Event::Obstacle(Obstacle::VeryWeakWall)))
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
    Chest(Treasure),
    BigChest(Treasure),
    HiddenChest(Treasure),
    LockedDoor,
    Entrance,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum EventTreeAction {
    AddEvent(Event),
    PrependAny(Event),
    PrependEach(Event),
    PrependGrouped(Event),
    HideChests,
}

impl Action for EventTreeAction {
    type Item = Event;
    fn apply<R: Rng>(&self, rng: &mut R, heads: &mut Vec<Tree<Event>>) {
        use rand::prelude::SliceRandom;
        use EventTreeAction::*;

        match self {
            AddEvent(event) => {
                heads.push(Tree::new().prepended(*event));
            }
            PrependAny(event) => {
                heads.choose_mut(rng).unwrap().prepend(*event);
            }
            PrependEach(event) => {
                for head in heads {
                    head.prepend(*event);
                }
            }
            PrependGrouped(event) => {
                let group = heads.drain(..).collect();
                let new_head = Tree::Branch(group).prepended(*event);
                heads.push(new_head)
            }
            HideChests => {
                fn accept(node: &mut Tree<Event>) {
                    let treasure = match node {
                        Tree::Event(event, next) => {
                            accept(next);
                            if let Event::Chest(treasure) = event {
                                Some(*treasure)
                            } else {
                                None
                            }
                        }
                        Tree::Branch(nodes) => {
                            for node in nodes {
                                accept(node);
                            }
                            None
                        }
                    };
                    if let Some(treasure) = treasure {
                        node.skip_event();
                        node.prepend(Event::HiddenChest(treasure));
                    }
                }
                for head in heads {
                    accept(head);
                }
            }
        }
    }
}

pub fn calc_join_weight(
    tree: &Tree<Event>,
    max_depth: usize,
) -> Box<dyn Fn(&Tree<Event>) -> usize> {
    let max_score = max_depth + 1;
    fn big_key_pred(event: &Event) -> bool {
        matches!(
            event,
            Event::Boss
                | Event::BigChest(_)
                | Event::HiddenChest(Treasure::BigKey)
                | Event::Chest(Treasure::BigKey)
        )
    }
    let depth = tree.find_event_depth(&big_key_pred);
    if depth.is_some() {
        Box::new(move |node| node.find_event_depth(&big_key_pred).unwrap_or(max_score))
    } else {
        Box::new(move |node| {
            let depth = node.find_event_depth(&big_key_pred).unwrap_or(max_score);
            if depth > max_score {
                println!("max {} depth {}", max_score, depth);
                node.show();
            }
            max_score - depth
        })
    }
}
